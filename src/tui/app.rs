use crate::config::{config::Config, session};
use crate::engine::{
    base::{Base, HexStyle},
    error::CalcError,
    ops,
    stack::CalcState,
    undo::UndoHistory,
    value::CalcValue,
};
use crate::input::{action::Action, commands::parse_command, mode::AppMode, parser::parse_value};
use arboard::Clipboard;

fn yank_text(val: &CalcValue, base: Base, hex_style: HexStyle) -> String {
    let raw = val.display_with_base(base);
    if base != Base::Hex || hex_style == HexStyle::ZeroX {
        return raw;
    }
    // raw is "0xABCD", "-0xABCD", or "0" (integer zero edge case)
    let negative = raw.starts_with('-');
    let sign = if negative { "-" } else { "" };
    let hex_part = if negative {
        raw.get(3..).unwrap_or(&raw) // strip "-0x"
    } else if raw.starts_with("0x") {
        raw.get(2..).unwrap_or(&raw) // strip "0x"
    } else {
        return raw; // bare "0" for integer zero
    };
    match hex_style {
        HexStyle::ZeroX => unreachable!("ZeroX handled by early return above"),
        HexStyle::Dollar => format!("{}${}", sign, hex_part),
        HexStyle::Hash => format!("{}#{}", sign, hex_part),
        HexStyle::Suffix => format!("{}{}h", sign, hex_part),
    }
}

pub struct App {
    pub state: CalcState,
    pub undo_history: UndoHistory,
    pub mode: AppMode,
    pub error_message: Option<String>,
    pub should_quit: bool,
    pub last_command: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let mut state = CalcState::new();
        state.angle_mode = config.angle_mode; // config default; overridden by session restore
        state.base = config.base; // config default; overridden by session restore
        state.precision = config.precision; // config default; overridden by session restore
        state.notation = config.notation; // config default; overridden by session restore
        Self {
            state,
            undo_history: UndoHistory::with_max_depth(config.max_undo_history),
            mode: AppMode::Normal,
            error_message: None,
            should_quit: false,
            last_command: None,
        }
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
                self.error_message = None;
            }
            Action::Noop => {}
            Action::Undo => {
                if let Some(prev) = self.undo_history.undo(&self.state) {
                    self.state = prev;
                    self.error_message = None;
                } else {
                    self.error_message = Some("Nothing to undo".into());
                }
            }
            Action::Redo => {
                if let Some(next) = self.undo_history.redo(&self.state) {
                    self.state = next;
                    self.error_message = None;
                } else {
                    self.error_message = Some("Nothing to redo".into());
                }
            }
            Action::EnterAlphaMode => {
                self.mode = AppMode::Alpha(String::new());
                self.error_message = None;
            }
            Action::InsertChar(c) => {
                let pushed = match &mut self.mode {
                    AppMode::Insert(buf) => {
                        buf.push(c);
                        true
                    }
                    _ => false,
                };
                if !pushed {
                    self.mode = AppMode::Insert(c.to_string());
                }
                self.error_message = None;
            }
            Action::InsertSubmit => {
                let mode = std::mem::replace(&mut self.mode, AppMode::Normal);
                if let AppMode::Insert(buf) = mode {
                    self.error_message = None;
                    if buf.is_empty() {
                        return;
                    }
                    if let Ok(val) = parse_value(&buf) {
                        let pre_op = self.state.clone();
                        self.state.push(val);
                        self.undo_history.snapshot(&pre_op);
                    } else {
                        self.error_message =
                            Some(format!("Cannot parse: {} (expected a number)", buf));
                    }
                }
            }
            Action::InsertSubmitThen(op) => {
                let buf = match &self.mode {
                    AppMode::Insert(buf) => buf.clone(),
                    _ => String::new(),
                };
                self.mode = AppMode::Normal;
                self.error_message = None;

                if !buf.is_empty() {
                    if let Ok(val) = parse_value(&buf) {
                        let pre_op = self.state.clone();
                        self.state.push(val);
                        self.undo_history.snapshot(&pre_op);
                    } else {
                        self.error_message = Some(format!("Cannot parse: {}", buf));
                        return;
                    }
                }

                let pre_op = self.state.clone();
                match self.dispatch(Action::Execute(op)) {
                    Ok(()) => {
                        self.undo_history.snapshot(&pre_op);
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                    }
                }
            }
            Action::InsertBackspace => {
                let should_reset = match &mut self.mode {
                    AppMode::Insert(buf) => {
                        buf.pop();
                        buf.is_empty()
                    }
                    _ => false,
                };
                if should_reset {
                    self.mode = AppMode::Normal;
                }
                self.error_message = None;
            }
            Action::InsertCancel => {
                self.mode = AppMode::Normal;
                self.error_message = None;
            }
            Action::EnterStoreMode => {
                if self.state.stack.is_empty() {
                    self.error_message = Some("Cannot store: stack is empty".to_string());
                } else {
                    self.mode = AppMode::AlphaStore(String::new());
                    self.error_message = None;
                }
            }
            Action::AlphaChar(c) => {
                match &mut self.mode {
                    AppMode::Alpha(buf) | AppMode::AlphaStore(buf) => {
                        buf.push(c);
                    }
                    _ => {}
                }
                self.error_message = None;
            }
            Action::AlphaSubmit => {
                let mode = std::mem::replace(&mut self.mode, AppMode::Normal);
                match mode {
                    AppMode::AlphaStore(buf) => {
                        self.error_message = None;
                        let name = buf.trim().to_string();
                        if name.is_empty() {
                            self.error_message = Some("Register name cannot be empty".to_string());
                        } else {
                            match self.state.stack.last() {
                                None => {
                                    self.error_message =
                                        Some("Cannot store: stack is empty".to_string());
                                }
                                Some(val) => {
                                    let pre_op = self.state.clone();
                                    let val = val.clone();
                                    self.state.registers.insert(name, val);
                                    self.undo_history.snapshot(&pre_op);
                                }
                            }
                        }
                    }
                    AppMode::Alpha(buf) => {
                        self.error_message = None;
                        if buf.is_empty() {
                            return;
                        }
                        match parse_command(&buf) {
                            Ok(action) => {
                                let pre_op = self.state.clone();
                                match self.dispatch(action) {
                                    Ok(()) => {
                                        self.undo_history.snapshot(&pre_op);
                                    }
                                    Err(e) => {
                                        self.error_message = Some(e.to_string());
                                    }
                                }
                            }
                            Err(_) => {
                                self.error_message = Some(format!(
                                    "Unknown command: {} (use 'name STORE', 'name RCL', or 'name DEL')",
                                    buf
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Action::AlphaBackspace => {
                let should_reset = match &mut self.mode {
                    AppMode::Alpha(buf) | AppMode::AlphaStore(buf) => {
                        buf.pop();
                        buf.is_empty()
                    }
                    _ => false,
                };
                if should_reset {
                    self.mode = AppMode::Normal;
                }
                self.error_message = None;
            }
            Action::AlphaCancel => {
                self.mode = AppMode::Normal;
                self.error_message = None;
            }
            Action::EnterChordMode(cat) => {
                self.mode = AppMode::Chord(cat);
                self.error_message = None;
            }
            Action::ChordCancel => {
                self.mode = AppMode::Normal;
                self.error_message = None;
            }
            Action::ChordInvalid => {
                self.mode = AppMode::Normal;
                self.error_message = Some("Unknown chord key".into());
            }
            Action::EnterBrowseMode => {
                if self.state.stack.len() < 2 {
                    self.error_message =
                        Some("stack underflow: roll requires at least 2 items".into());
                } else {
                    self.mode = AppMode::Browse(2);
                    self.error_message = None;
                }
            }
            Action::BrowseCursorUp => {
                if let AppMode::Browse(pos) = &mut self.mode {
                    let max_pos = self.state.stack.len();
                    if *pos < max_pos {
                        *pos += 1;
                    }
                }
            }
            Action::BrowseCursorDown => {
                if let AppMode::Browse(pos) = &mut self.mode {
                    if *pos > 2 {
                        *pos -= 1;
                    }
                }
            }
            Action::BrowseConfirm => {
                if let AppMode::Browse(pos) = self.mode {
                    self.mode = AppMode::Normal;
                    let pre_op = self.state.clone();
                    match self.state.roll(pos) {
                        Ok(()) => {
                            self.undo_history.snapshot(&pre_op);
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(e.to_string());
                        }
                    }
                }
            }
            Action::BrowseCancel => {
                self.mode = AppMode::Normal;
                self.error_message = None;
            }
            Action::EnterPrecisionInput => {
                self.mode = AppMode::PrecisionInput(String::new());
                self.error_message = None;
            }
            Action::PrecisionDigit(c) => {
                if let AppMode::PrecisionInput(buf) = &mut self.mode {
                    if buf.len() < 2 {
                        buf.push(c);
                    }
                }
            }
            Action::PrecisionBackspace => {
                if let AppMode::PrecisionInput(buf) = &mut self.mode {
                    buf.pop();
                }
            }
            Action::PrecisionSubmit => {
                let mode = std::mem::replace(&mut self.mode, AppMode::Normal);
                if let AppMode::PrecisionInput(buf) = mode {
                    if !buf.is_empty() {
                        match buf.parse::<usize>() {
                            Ok(p) if p >= 1 && p <= 15 => {
                                self.state.precision = p;
                                self.error_message = None;
                            }
                            _ => {
                                self.error_message =
                                    Some(format!("Precision must be 1–15, got '{}'", buf));
                            }
                        }
                    }
                }
            }
            Action::PrecisionCancel => {
                self.mode = AppMode::Normal;
                self.error_message = None;
            }
            Action::Yank => match self.state.stack.last() {
                None => {
                    self.error_message = Some("Stack is empty".into());
                }
                Some(val) => {
                    let text = yank_text(val, self.state.base, self.state.hex_style);
                    match Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
                        Ok(()) => {
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Clipboard error: {}", e));
                        }
                    }
                }
            },
            action => {
                let was_chord = matches!(self.mode, AppMode::Chord(_));
                let pre_op = self.state.clone();
                match self.dispatch(action) {
                    Ok(()) => {
                        self.undo_history.snapshot(&pre_op);
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                    }
                }
                if was_chord {
                    self.mode = AppMode::Normal;
                }
            }
        }
    }

    fn dispatch(&mut self, action: Action) -> Result<(), CalcError> {
        match action {
            Action::Push(v) => {
                self.state.push(v);
                Ok(())
            }
            Action::Execute(op) => ops::apply_op(&mut self.state, op),
            Action::SetBase(b) => {
                self.state.base = b;
                Ok(())
            }
            Action::SetAngleMode(m) => {
                self.state.angle_mode = m;
                Ok(())
            }
            Action::SetHexStyle(s) => {
                if self.state.base != Base::Hex {
                    return Err(CalcError::InvalidInput(
                        "Hex style is only available in HEX base".to_string(),
                    ));
                }
                self.state.hex_style = s;
                Ok(())
            }
            Action::SetNotation(n) => {
                self.state.notation = n;
                Ok(())
            }
            Action::StoreRegister(name) => {
                let val = self.state.stack.pop().ok_or(CalcError::StackUnderflow)?;
                self.state.registers.insert(name, val);
                Ok(())
            }
            Action::RecallRegister(name) => {
                let val = self
                    .state
                    .registers
                    .get(&name)
                    .ok_or_else(|| {
                        CalcError::InvalidInput(format!("Register '{}' not found", name))
                    })?
                    .clone();
                self.state.stack.push(val);
                Ok(())
            }
            Action::DeleteRegister(name) => {
                self.state.registers.remove(&name).ok_or_else(|| {
                    CalcError::InvalidInput(format!("Register '{}' not found", name))
                })?;
                Ok(())
            }
            Action::ResetSession => {
                self.state = CalcState::new();
                let _ = session::save(&self.state);
                Ok(())
            }
            Action::Quit
            | Action::Noop
            | Action::Undo
            | Action::Redo
            | Action::EnterAlphaMode
            | Action::EnterStoreMode
            | Action::InsertChar(_)
            | Action::InsertBackspace
            | Action::InsertSubmit
            | Action::InsertSubmitThen(_)
            | Action::InsertCancel
            | Action::AlphaChar(_)
            | Action::AlphaBackspace
            | Action::AlphaSubmit
            | Action::AlphaCancel
            | Action::EnterChordMode(_)
            | Action::ChordCancel
            | Action::ChordInvalid
            | Action::Yank
            | Action::EnterBrowseMode
            | Action::BrowseCursorUp
            | Action::BrowseCursorDown
            | Action::BrowseConfirm
            | Action::BrowseCancel
            | Action::EnterPrecisionInput
            | Action::PrecisionDigit(_)
            | Action::PrecisionBackspace
            | Action::PrecisionSubmit
            | Action::PrecisionCancel => unreachable!("handled in apply()"),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{
        angle::AngleMode,
        base::{Base, HexStyle},
        ops::Op,
        value::CalcValue,
    };
    use crate::input::mode::ChordCategory;
    use dashu::integer::IBig;

    fn push_int(app: &mut App, n: i32) {
        app.apply(Action::Push(CalcValue::Integer(IBig::from(n))));
    }

    #[test]
    fn test_quit_sets_should_quit() {
        let mut app = App::new();
        app.apply(Action::Quit);
        assert!(app.should_quit);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_noop_does_nothing() {
        let mut app = App::new();
        app.apply(Action::Noop);
        assert!(!app.should_quit);
        assert!(app.error_message.is_none());
        assert_eq!(app.state.depth(), 0);
    }

    #[test]
    fn test_push_adds_to_stack() {
        let mut app = App::new();
        push_int(&mut app, 42);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_push_snapshots_undo_history() {
        let mut app = App::new();
        push_int(&mut app, 1);
        assert!(app.undo_history.can_undo());
    }

    #[test]
    fn test_undo_restores_state() {
        let mut app = App::new();
        push_int(&mut app, 10);
        push_int(&mut app, 20);
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_undo_nothing_sets_error() {
        let mut app = App::new();
        app.apply(Action::Undo);
        assert_eq!(app.error_message.as_deref(), Some("Nothing to undo"));
    }

    #[test]
    fn test_redo_nothing_sets_error() {
        let mut app = App::new();
        app.apply(Action::Redo);
        assert_eq!(app.error_message.as_deref(), Some("Nothing to redo"));
    }

    #[test]
    fn test_redo_after_undo() {
        let mut app = App::new();
        push_int(&mut app, 5);
        app.apply(Action::Undo);
        app.apply(Action::Redo);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_engine_error_sets_error_message() {
        let mut app = App::new();
        // Execute op on empty stack → StackUnderflow
        app.apply(Action::Execute(Op::Add));
        assert!(app.error_message.is_some());
        // Stack must remain unchanged (atomicity)
        assert_eq!(app.state.depth(), 0);
    }

    #[test]
    fn test_engine_error_does_not_snapshot() {
        let mut app = App::new();
        app.apply(Action::Execute(Op::Add));
        assert!(!app.undo_history.can_undo());
    }


    #[test]
    fn test_store_register() {
        let mut app = App::new();
        push_int(&mut app, 99);
        app.apply(Action::StoreRegister("x".into()));
        assert_eq!(app.state.depth(), 0);
        assert!(app.state.registers.contains_key("x"));
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_store_register_empty_stack_error() {
        let mut app = App::new();
        app.apply(Action::StoreRegister("x".into()));
        assert!(app.error_message.is_some());
    }

    #[test]
    fn test_recall_register() {
        let mut app = App::new();
        push_int(&mut app, 7);
        app.apply(Action::StoreRegister("r".into()));
        app.apply(Action::RecallRegister("r".into()));
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_recall_missing_register_error() {
        let mut app = App::new();
        app.apply(Action::RecallRegister("z".into()));
        assert!(app.error_message.is_some());
    }

    #[test]
    fn test_delete_register() {
        let mut app = App::new();
        push_int(&mut app, 3);
        app.apply(Action::StoreRegister("d".into()));
        app.apply(Action::DeleteRegister("d".into()));
        assert!(!app.state.registers.contains_key("d"));
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_delete_missing_register_error() {
        let mut app = App::new();
        app.apply(Action::DeleteRegister("nope".into()));
        assert!(app.error_message.is_some());
    }

    #[test]
    fn test_error_cleared_on_success() {
        let mut app = App::new();
        app.apply(Action::Execute(Op::Add)); // sets error
        assert!(app.error_message.is_some());
        push_int(&mut app, 1); // success clears error
        assert!(app.error_message.is_none());
    }

    // EnterAlphaMode → transitions to true Alpha mode (i key)
    #[test]
    fn test_enter_alpha_mode_creates_alpha() {
        let mut app = App::new();
        app.apply(Action::EnterAlphaMode);
        assert_eq!(app.mode, AppMode::Alpha(String::new()));
        assert!(app.error_message.is_none());
    }

    // InsertChar in Normal → creates Insert mode with that char
    #[test]
    fn test_insert_char_in_normal_creates_insert_mode() {
        let mut app = App::new();
        app.apply(Action::InsertChar('5'));
        assert_eq!(app.mode, AppMode::Insert("5".into()));
        assert!(app.error_message.is_none());
    }

    // InsertChar in Insert mode → appends to buffer
    #[test]
    fn test_insert_char_appends_to_buffer() {
        let mut app = App::new();
        app.apply(Action::InsertChar('1'));
        app.apply(Action::InsertChar('2'));
        assert_eq!(app.mode, AppMode::Insert("12".into()));
    }

    // InsertSubmit with integer buffer → pushes value, returns to Normal
    #[test]
    fn test_insert_submit_pushes_integer() {
        let mut app = App::new();
        app.mode = AppMode::Insert("42".into());
        app.apply(Action::InsertSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    // InsertSubmit with float buffer → pushes value, returns to Normal
    #[test]
    fn test_insert_submit_pushes_float() {
        let mut app = App::new();
        app.mode = AppMode::Insert("3.14".into());
        app.apply(Action::InsertSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    // InsertSubmit with empty buffer → returns to Normal, stack unchanged
    #[test]
    fn test_insert_submit_empty_buffer_returns_to_normal() {
        let mut app = App::new();
        app.mode = AppMode::Insert(String::new());
        app.apply(Action::InsertSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 0);
        assert!(app.error_message.is_none());
    }

    // InsertSubmit with non-numeric input → error (Insert mode only parses numbers)
    #[test]
    fn test_insert_submit_non_numeric_sets_error() {
        let mut app = App::new();
        app.mode = AppMode::Insert("garbage".into());
        app.apply(Action::InsertSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_some());
    }

    // InsertCancel → returns to Normal, stack unchanged
    #[test]
    fn test_insert_cancel_returns_to_normal() {
        let mut app = App::new();
        push_int(&mut app, 7);
        app.mode = AppMode::Insert("typing".into());
        app.apply(Action::InsertCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    // InsertSubmit snapshots undo history on successful push
    #[test]
    fn test_insert_submit_snapshots_undo() {
        let mut app = App::new();
        app.mode = AppMode::Insert("10".into());
        app.apply(Action::InsertSubmit);
        assert!(app.undo_history.can_undo());
    }

    // AlphaSubmit command dispatch — STORE command (via Alpha mode)
    #[test]
    fn test_alpha_submit_store_command() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.mode = AppMode::Alpha("myvar STORE".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 0);
        assert!(app.state.registers.contains_key("myvar"));
        assert!(app.error_message.is_none());
    }

    // AlphaSubmit command dispatch — RCL command (via Alpha mode)
    #[test]
    fn test_alpha_submit_recall_command() {
        let mut app = App::new();
        push_int(&mut app, 7);
        app.apply(Action::StoreRegister("r1".into()));
        app.mode = AppMode::Alpha("r1 RCL".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    // Alpha mode: unrecognized command sets error
    #[test]
    fn test_alpha_submit_unknown_command_sets_error() {
        let mut app = App::new();
        app.mode = AppMode::Alpha("garbage".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_some());
    }

    // AlphaCancel in Alpha mode → returns to Normal
    #[test]
    fn test_alpha_cancel_returns_to_normal() {
        let mut app = App::new();
        push_int(&mut app, 7);
        app.mode = AppMode::Alpha("r1".into());
        app.apply(Action::AlphaCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 1);
        assert!(app.error_message.is_none());
    }

    // AC 4: Yank on empty stack sets error
    #[test]
    fn test_yank_empty_stack_sets_error() {
        let mut app = App::new();
        app.apply(Action::Yank);
        assert_eq!(app.error_message.as_deref(), Some("Stack is empty"));
    }

    // AC 1: Yank does not modify the stack
    #[test]
    fn test_yank_preserves_stack() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.apply(Action::Yank);
        assert_eq!(app.state.depth(), 1);
        // error_message may be set if clipboard unavailable in CI — that's OK
    }

    // AC 2: DEC base → plain decimal string
    #[test]
    fn test_yank_text_dec() {
        let val = CalcValue::Integer(IBig::from(42));
        assert_eq!(yank_text(&val, Base::Dec, HexStyle::ZeroX), "42");
    }

    // AC 3: HEX + ZeroX → "0xFF"
    #[test]
    fn test_yank_text_hex_zerox() {
        let val = CalcValue::Integer(IBig::from(255));
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::ZeroX), "0xFF");
    }

    // AC 3: HEX + Dollar → "$FF"
    #[test]
    fn test_yank_text_hex_dollar() {
        let val = CalcValue::Integer(IBig::from(255));
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::Dollar), "$FF");
    }

    // AC 3: HEX + Hash → "#FF"
    #[test]
    fn test_yank_text_hex_hash() {
        let val = CalcValue::Integer(IBig::from(255));
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::Hash), "#FF");
    }

    // AC 3: HEX + Suffix → "FFh"
    #[test]
    fn test_yank_text_hex_suffix() {
        let val = CalcValue::Integer(IBig::from(255));
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::Suffix), "FFh");
    }

    // AC 2: float in DEC base → decimal string
    #[test]
    fn test_yank_text_float() {
        let val = CalcValue::from_f64(3.14);
        let result = yank_text(&val, Base::Dec, HexStyle::ZeroX);
        assert!(
            result.starts_with("3.14"),
            "expected '3.14...', got '{}'",
            result
        );
    }

    // Edge case: integer zero in HEX → "0" (no prefix transformation)
    #[test]
    fn test_yank_text_zero_hex() {
        let val = CalcValue::Integer(IBig::from(0));
        // zero displays as bare "0" regardless of HexStyle
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::Dollar), "0");
    }

    // Edge case: negative integer in HEX + Dollar style
    #[test]
    fn test_yank_text_negative_hex_dollar() {
        let val = CalcValue::Integer(IBig::from(-255));
        assert_eq!(yank_text(&val, Base::Hex, HexStyle::Dollar), "-$FF");
    }

    #[test]
    fn test_yank_text_oct() {
        let val = CalcValue::Integer(IBig::from(8));
        assert_eq!(yank_text(&val, Base::Oct, HexStyle::ZeroX), "0o10");
    }

    #[test]
    fn test_yank_text_bin() {
        let val = CalcValue::Integer(IBig::from(5));
        assert_eq!(yank_text(&val, Base::Bin, HexStyle::ZeroX), "0b101");
    }

    // Story 3.3 — AC 1/2: SetBase(Hex) updates state.base
    #[test]
    fn test_set_base_hex_updates_state() {
        let mut app = App::new();
        app.apply(Action::SetBase(Base::Hex));
        assert_eq!(app.state.base, Base::Hex);
        assert!(app.error_message.is_none());
    }

    // Story 3.3 — AC 2: all four base variants update state.base
    #[test]
    fn test_set_base_all_variants() {
        for base in [Base::Dec, Base::Hex, Base::Oct, Base::Bin] {
            let mut app = App::new();
            app.apply(Action::SetBase(base));
            assert_eq!(
                app.state.base, base,
                "SetBase({:?}) should update state.base",
                base
            );
        }
    }

    // Story 3.3 — AC 1: SetBase creates an undo snapshot
    #[test]
    fn test_set_base_snapshots_undo() {
        let mut app = App::new();
        app.apply(Action::SetBase(Base::Hex));
        assert!(
            app.undo_history.can_undo(),
            "SetBase should create an undo snapshot"
        );
    }

    // Story 3.3 — AC 1: Undo after SetBase restores previous base
    #[test]
    fn test_set_base_undo_restores_previous() {
        let mut app = App::new();
        assert_eq!(app.state.base, Base::Dec);
        app.apply(Action::SetBase(Base::Hex));
        app.apply(Action::Undo);
        assert_eq!(
            app.state.base,
            Base::Dec,
            "Undo should restore Dec after SetBase(Hex)"
        );
    }

    // Story 3.3 — AC 3: SetAngleMode(Rad) updates state.angle_mode
    #[test]
    fn test_set_angle_mode_rad_updates_state() {
        let mut app = App::new();
        app.apply(Action::SetAngleMode(AngleMode::Rad));
        assert_eq!(app.state.angle_mode, AngleMode::Rad);
        assert!(app.error_message.is_none());
    }

    // Story 3.3 — AC 3: angle mode gates trig — sin(90) differs between DEG and RAD
    #[test]
    fn test_set_angle_mode_affects_trig() {
        // DEG: sin(90°) = 1.0
        let mut app_deg = App::new();
        app_deg.apply(Action::SetAngleMode(AngleMode::Deg));
        app_deg.apply(Action::Push(CalcValue::from_f64(90.0)));
        app_deg.apply(Action::Execute(Op::Sin));
        assert!(
            app_deg.error_message.is_none(),
            "sin should succeed in DEG mode"
        );
        let deg_result = app_deg
            .state
            .stack
            .last()
            .unwrap()
            .display_with_base(Base::Dec);

        // RAD: sin(90 rad) ≠ 1.0
        let mut app_rad = App::new();
        app_rad.apply(Action::SetAngleMode(AngleMode::Rad));
        app_rad.apply(Action::Push(CalcValue::from_f64(90.0)));
        app_rad.apply(Action::Execute(Op::Sin));
        assert!(
            app_rad.error_message.is_none(),
            "sin should succeed in RAD mode"
        );
        let rad_result = app_rad
            .state
            .stack
            .last()
            .unwrap()
            .display_with_base(Base::Dec);

        assert_ne!(
            deg_result, rad_result,
            "sin(90) must differ between DEG ({}) and RAD ({}) modes",
            deg_result, rad_result
        );
    }

    // Story 3.3 — AC 4: SetHexStyle(Dollar) updates state.hex_style
    #[test]
    fn test_set_hex_style_updates_state() {
        let mut app = App::new();
        app.state.base = Base::Hex; // hex style only valid in HEX base
        app.apply(Action::SetHexStyle(HexStyle::Dollar));
        assert_eq!(app.state.hex_style, HexStyle::Dollar);
        assert!(app.error_message.is_none());
    }

    // Story 3.3 — AC 4: all four HexStyle variants update state.hex_style
    #[test]
    fn test_set_hex_style_all_variants() {
        for style in [
            HexStyle::ZeroX,
            HexStyle::Dollar,
            HexStyle::Hash,
            HexStyle::Suffix,
        ] {
            let mut app = App::new();
            app.state.base = Base::Hex; // hex style only valid in HEX base
            app.apply(Action::SetHexStyle(style));
            assert_eq!(
                app.state.hex_style, style,
                "SetHexStyle({:?}) should update state.hex_style",
                style
            );
        }
    }

    // Story 3.3 — AC 1/3/4: mode-change actions clear error_message
    #[test]
    fn test_mode_changes_clear_error() {
        // SetBase clears error
        let mut app = App::new();
        app.apply(Action::Execute(Op::Add)); // stack underflow → sets error
        assert!(app.error_message.is_some());
        app.apply(Action::SetBase(Base::Hex));
        assert!(
            app.error_message.is_none(),
            "SetBase should clear error_message"
        );

        // SetAngleMode clears error
        let mut app2 = App::new();
        app2.apply(Action::Execute(Op::Add));
        assert!(app2.error_message.is_some());
        app2.apply(Action::SetAngleMode(AngleMode::Rad));
        assert!(
            app2.error_message.is_none(),
            "SetAngleMode should clear error_message"
        );

        // SetHexStyle clears error (must be in HEX base)
        let mut app3 = App::new();
        app3.state.base = Base::Hex;
        app3.apply(Action::Execute(Op::Add));
        assert!(app3.error_message.is_some());
        app3.apply(Action::SetHexStyle(HexStyle::Hash));
        assert!(
            app3.error_message.is_none(),
            "SetHexStyle should clear error_message"
        );
    }

    // AC 1: EnterChordMode sets Chord mode
    #[test]
    fn test_enter_chord_mode() {
        use crate::input::mode::ChordCategory;
        let mut app = App::new();
        app.apply(Action::EnterChordMode(ChordCategory::Trig));
        assert_eq!(app.mode, AppMode::Chord(ChordCategory::Trig));
        assert!(app.error_message.is_none());
    }

    // AC 3: ChordCancel returns to Normal, no error
    #[test]
    fn test_chord_cancel_returns_to_normal() {
        use crate::input::mode::ChordCategory;
        let mut app = App::new();
        app.mode = AppMode::Chord(ChordCategory::Log);
        app.apply(Action::ChordCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_none());
    }

    // AC 4: ChordInvalid returns to Normal with error
    #[test]
    fn test_chord_invalid_sets_error() {
        use crate::input::mode::ChordCategory;
        let mut app = App::new();
        app.mode = AppMode::Chord(ChordCategory::Trig);
        app.apply(Action::ChordInvalid);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_some());
    }

    // AC 2: chord execute returns to Normal on success
    #[test]
    fn test_chord_execute_returns_to_normal() {
        use crate::input::mode::ChordCategory;
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::Chord(ChordCategory::Trig);
        app.apply(Action::Execute(Op::Sin));
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_none());
    }

    // AC 2+4: chord execute failure returns to Normal with error
    #[test]
    fn test_chord_execute_failure_returns_to_normal() {
        use crate::input::mode::ChordCategory;
        let mut app = App::new();
        // Empty stack → Sin will fail
        app.mode = AppMode::Chord(ChordCategory::Trig);
        app.apply(Action::Execute(Op::Sin));
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_some());
    }

    // ── Story 4.1: Named Memory Registers ────────────────────────────────────

    // AC 8: EnterStoreMode with empty stack → error, mode stays Normal
    #[test]
    fn test_enter_store_mode_empty_stack_error() {
        let mut app = App::new();
        app.apply(Action::EnterStoreMode);
        assert!(matches!(app.mode, AppMode::Normal));
        assert_eq!(
            app.error_message.as_deref(),
            Some("Cannot store: stack is empty")
        );
    }

    // AC 1: EnterStoreMode with value on stack → mode becomes AlphaStore("")
    #[test]
    fn test_enter_store_mode_with_value() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.apply(Action::EnterStoreMode);
        assert!(matches!(app.mode, AppMode::AlphaStore(ref s) if s.is_empty()));
        assert!(app.error_message.is_none());
    }

    // AlphaChar appends to AlphaStore buffer
    #[test]
    fn test_alpha_char_appends_in_alpha_store() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::AlphaStore(String::new());
        app.apply(Action::AlphaChar('m'));
        app.apply(Action::AlphaChar('y'));
        assert!(matches!(app.mode, AppMode::AlphaStore(ref s) if s == "my"));
    }

    // AlphaBackspace works in AlphaStore mode
    #[test]
    fn test_alpha_backspace_in_alpha_store() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::AlphaStore("my".into());
        app.apply(Action::AlphaBackspace);
        assert!(matches!(app.mode, AppMode::AlphaStore(ref s) if s == "m"));
    }

    // AlphaBackspace in AlphaStore with single char → returns to Normal
    #[test]
    fn test_alpha_backspace_empties_alpha_store_returns_normal() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::AlphaStore("x".into());
        app.apply(Action::AlphaBackspace);
        assert_eq!(app.mode, AppMode::Normal);
    }

    // AlphaCancel in AlphaStore → returns to Normal
    #[test]
    fn test_alpha_cancel_in_alpha_store() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::AlphaStore("partial".into());
        app.apply(Action::AlphaCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_none());
    }

    // AC 2: AlphaSubmit in AlphaStore → register stored (peek, stack unchanged), mode Normal
    #[test]
    fn test_alpha_store_submit_peeks_stack_unchanged() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.mode = AppMode::AlphaStore("x".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        // Stack is unchanged (peek, not pop)
        assert_eq!(app.state.depth(), 1);
        assert!(app.state.registers.contains_key("x"));
        assert!(app.error_message.is_none());
    }

    // AlphaStore submit creates undo snapshot
    #[test]
    fn test_alpha_store_submit_snapshots_undo() {
        let mut app = App::new();
        push_int(&mut app, 7);
        app.mode = AppMode::AlphaStore("r1".into());
        app.apply(Action::AlphaSubmit);
        // There is at least one undo entry
        assert!(app.undo_history.can_undo());
    }

    // AlphaStore submit with empty name → error
    #[test]
    fn test_alpha_store_submit_empty_name_error() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.mode = AppMode::AlphaStore(String::new());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_some());
    }

    // AC 7: AlphaSubmit with unrecognized input → improved error message
    #[test]
    fn test_alpha_submit_invalid_improved_error_message() {
        let mut app = App::new();
        app.mode = AppMode::Alpha("hello".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        let msg = app.error_message.as_deref().unwrap_or("");
        assert!(
            msg.contains("STORE") && msg.contains("RCL"),
            "error should contain STORE/RCL hint, got: {}",
            msg
        );
    }

    // AC 3: AlphaSubmit with DEL command via Alpha mode → register deleted
    #[test]
    fn test_alpha_submit_del_command() {
        let mut app = App::new();
        push_int(&mut app, 5);
        app.apply(Action::StoreRegister("tmp".into()));
        assert!(app.state.registers.contains_key("tmp"));
        app.mode = AppMode::Alpha("tmp DEL".into());
        app.apply(Action::AlphaSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(!app.state.registers.contains_key("tmp"));
        assert!(app.error_message.is_none());
    }

    // ── Story 4.2: Full-State Undo & Redo ────────────────────────────────────

    // AC 1: StoreRegister creates undoable snapshot — undo removes register
    #[test]
    fn test_undo_register_store() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.apply(Action::StoreRegister("x".into()));
        assert!(app.state.registers.contains_key("x"));
        app.apply(Action::Undo);
        // StoreRegister pops from stack — undo restores: stack has 42, register gone
        assert_eq!(app.state.depth(), 1);
        assert!(!app.state.registers.contains_key("x"));
    }

    // AC 1: DeleteRegister creates undoable snapshot — undo restores the register
    #[test]
    fn test_undo_register_delete() {
        let mut app = App::new();
        push_int(&mut app, 5);
        app.apply(Action::StoreRegister("tmp".into()));
        app.apply(Action::DeleteRegister("tmp".into()));
        assert!(!app.state.registers.contains_key("tmp"));
        app.apply(Action::Undo);
        assert!(app.state.registers.contains_key("tmp"));
    }

    // AC 1: Single undo restores both stack and registers atomically
    #[test]
    fn test_undo_restores_registers_and_stack_atomically() {
        let mut app = App::new();
        push_int(&mut app, 10);
        push_int(&mut app, 20);
        // StoreRegister pops 20 from stack, inserts to registers
        app.apply(Action::StoreRegister("r".into()));
        assert_eq!(app.state.depth(), 1);
        assert!(app.state.registers.contains_key("r"));
        // Undo: stack depth 2, register gone
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 2);
        assert!(!app.state.registers.contains_key("r"));
    }

    // AC 3: Multiple sequential undos step back through history one at a time
    #[test]
    fn test_multiple_sequential_undos() {
        let mut app = App::new();
        push_int(&mut app, 1);
        push_int(&mut app, 2);
        push_int(&mut app, 3);
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 2);
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 1);
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 0);
        app.apply(Action::Undo);
        assert_eq!(app.error_message.as_deref(), Some("Nothing to undo"));
        assert_eq!(app.state.depth(), 0);
    }

    // AC 2 + 3: Undo then redo restores the full chain
    #[test]
    fn test_redo_after_multiple_undos() {
        let mut app = App::new();
        push_int(&mut app, 10);
        push_int(&mut app, 20);
        app.apply(Action::Undo);
        app.apply(Action::Undo);
        assert_eq!(app.state.depth(), 0);
        app.apply(Action::Redo);
        assert_eq!(app.state.depth(), 1);
        app.apply(Action::Redo);
        assert_eq!(app.state.depth(), 2);
        app.apply(Action::Redo);
        assert_eq!(app.error_message.as_deref(), Some("Nothing to redo"));
    }

    // AC 7: Depth limit discards oldest snapshots — App-level integration
    #[test]
    fn test_depth_limit_discards_oldest() {
        let mut app = App::new();
        app.undo_history = UndoHistory::with_max_depth(3);
        for i in 1..=5 {
            push_int(&mut app, i);
        }
        assert_eq!(app.state.depth(), 5);
        // Only 3 undos available (oldest 2 snapshots discarded)
        app.apply(Action::Undo);
        app.apply(Action::Undo);
        app.apply(Action::Undo);
        app.apply(Action::Undo);
        assert_eq!(
            app.error_message.as_deref(),
            Some("Nothing to undo"),
            "oldest snapshots should have been discarded"
        );
    }

    // ── Story 4.4: Configuration File ────────────────────────────────────────

    #[test]
    fn test_app_new_default_precision() {
        // Config::load() returns defaults (no config.toml in test env usually)
        // Regardless, App::new() must set precision from config (default = 15)
        let app = App::new();
        assert_eq!(app.state.precision, 15);
    }

    #[test]
    fn test_app_new_default_angle_mode() {
        let app = App::new();
        assert_eq!(app.state.angle_mode, AngleMode::Deg);
    }

    // ── Story 4.3: Session Persistence & SIGTERM Safety ───────────────────────

    // AC 8: ResetSession clears stack and registers
    #[test]
    fn test_reset_session_clears_state() {
        let mut app = App::new();
        push_int(&mut app, 10);
        push_int(&mut app, 20);
        app.apply(Action::StoreRegister("r".into()));
        assert!(app.state.registers.contains_key("r"));
        assert_eq!(app.state.depth(), 1);
        app.apply(Action::ResetSession);
        assert_eq!(app.state.depth(), 0, "stack should be empty after reset");
        assert!(
            app.state.registers.is_empty(),
            "registers should be empty after reset"
        );
    }

    // AC 8: ResetSession goes through dispatch() → snapshotted → undo restores pre-reset state
    #[test]
    fn test_reset_session_is_undoable() {
        let mut app = App::new();
        push_int(&mut app, 42);
        app.apply(Action::ResetSession);
        assert_eq!(app.state.depth(), 0);
        app.apply(Action::Undo);
        assert_eq!(
            app.state.depth(),
            1,
            "undo should restore the value present before reset"
        );
    }

    // ── Browse mode (roll-to-top) ────────────────────────────────────────────

    // AC-7: ↑ with ≤1 item → error, not Browse mode
    #[test]
    fn test_enter_browse_empty_stack_errors() {
        let mut app = App::new();
        app.apply(Action::EnterBrowseMode);
        assert!(app.error_message.is_some(), "should show error on empty stack");
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_enter_browse_one_item_errors() {
        let mut app = App::new();
        push_int(&mut app, 1);
        app.apply(Action::EnterBrowseMode);
        assert!(app.error_message.is_some());
        assert_eq!(app.mode, AppMode::Normal);
    }

    // Entry with ≥2 items → Browse mode at cursor position 2
    #[test]
    fn test_enter_browse_two_items_succeeds() {
        let mut app = App::new();
        push_int(&mut app, 1);
        push_int(&mut app, 2);
        app.apply(Action::EnterBrowseMode);
        assert_eq!(app.mode, AppMode::Browse(2));
        assert!(app.error_message.is_none());
    }

    // AC-3: cursor moves deeper with BrowseCursorUp
    #[test]
    fn test_browse_cursor_up_increments() {
        let mut app = App::new();
        for i in 1..=4 { push_int(&mut app, i); }
        app.apply(Action::EnterBrowseMode); // cursor at 2
        app.apply(Action::BrowseCursorUp);
        assert_eq!(app.mode, AppMode::Browse(3));
        app.apply(Action::BrowseCursorUp);
        assert_eq!(app.mode, AppMode::Browse(4));
    }

    // AC-5: cursor clamped at depth
    #[test]
    fn test_browse_cursor_up_clamped_at_depth() {
        let mut app = App::new();
        push_int(&mut app, 1);
        push_int(&mut app, 2);
        app.apply(Action::EnterBrowseMode); // cursor at 2 (= depth)
        app.apply(Action::BrowseCursorUp);  // already at max
        assert_eq!(app.mode, AppMode::Browse(2));
    }

    // AC-4: cursor moves toward top with BrowseCursorDown
    #[test]
    fn test_browse_cursor_down_decrements() {
        let mut app = App::new();
        for i in 1..=4 { push_int(&mut app, i); }
        app.apply(Action::EnterBrowseMode);
        app.apply(Action::BrowseCursorUp); // cursor at 3
        app.apply(Action::BrowseCursorDown);
        assert_eq!(app.mode, AppMode::Browse(2));
    }

    // AC-6: cursor clamped at position 2
    #[test]
    fn test_browse_cursor_down_clamped_at_2() {
        let mut app = App::new();
        push_int(&mut app, 1);
        push_int(&mut app, 2);
        app.apply(Action::EnterBrowseMode); // cursor at 2
        app.apply(Action::BrowseCursorDown); // already at min
        assert_eq!(app.mode, AppMode::Browse(2));
    }

    // AC-1: BrowseConfirm rolls cursor item to top
    #[test]
    fn test_browse_confirm_rolls_item_to_top() {
        let mut app = App::new();
        push_int(&mut app, 1); // pos 3
        push_int(&mut app, 2); // pos 2
        push_int(&mut app, 3); // pos 1 (top)
        app.apply(Action::EnterBrowseMode); // cursor at 2
        app.apply(Action::BrowseConfirm);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.depth(), 3);
        // Item from pos 2 (value 2) is now at top
        assert_eq!(app.state.peek(), Some(&CalcValue::Integer(IBig::from(2))));
    }

    // AC-2: BrowseCancel exits without mutation
    #[test]
    fn test_browse_cancel_preserves_stack() {
        let mut app = App::new();
        push_int(&mut app, 10);
        push_int(&mut app, 20);
        let before = app.state.stack.clone();
        app.apply(Action::EnterBrowseMode);
        app.apply(Action::BrowseCursorUp);
        app.apply(Action::BrowseCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.stack, before);
    }

    // AC-12: roll is undo-able
    #[test]
    fn test_browse_roll_is_undoable() {
        let mut app = App::new();
        push_int(&mut app, 1);
        push_int(&mut app, 2);
        push_int(&mut app, 3);
        let before = app.state.stack.clone();
        app.apply(Action::EnterBrowseMode);
        app.apply(Action::BrowseConfirm); // rolls pos 2 to top
        assert_ne!(app.state.stack, before);
        app.apply(Action::Undo);
        assert_eq!(app.state.stack, before, "undo should restore pre-roll state");
    }

    // ── configure-settings-chord ────────────────────────────────────────────

    // AC-1: angle mode changes via C› chord (d → DEG)
    #[test]
    fn test_set_angle_mode_via_config_chord() {
        let mut app = App::new();
        app.apply(Action::EnterChordMode(ChordCategory::Config));
        app.apply(Action::SetAngleMode(AngleMode::Rad));
        assert_eq!(app.state.angle_mode, AngleMode::Rad);
        assert_eq!(app.mode, AppMode::Normal); // chord exited
        assert!(app.error_message.is_none());
    }

    // AC-2: base changes via C› chord (h → HEX)
    #[test]
    fn test_set_base_via_config_chord() {
        let mut app = App::new();
        app.apply(Action::EnterChordMode(ChordCategory::Config));
        app.apply(Action::SetBase(Base::Hex));
        assert_eq!(app.state.base, Base::Hex);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_none());
    }

    // AC-3: notation switches to sci
    #[test]
    fn test_set_notation_sci() {
        use crate::engine::notation::Notation;
        let mut app = App::new();
        app.apply(Action::EnterChordMode(ChordCategory::Config));
        app.apply(Action::SetNotation(Notation::Sci));
        assert_eq!(app.state.notation, Notation::Sci);
        assert_eq!(app.mode, AppMode::Normal);
        assert!(app.error_message.is_none());
    }

    // AC-4: notation switches to fixed
    #[test]
    fn test_set_notation_fixed() {
        use crate::engine::notation::Notation;
        let mut app = App::new();
        app.state.notation = Notation::Sci;
        app.apply(Action::SetNotation(Notation::Fixed));
        assert_eq!(app.state.notation, Notation::Fixed);
    }

    // AC-5: notation auto mode
    #[test]
    fn test_set_notation_auto() {
        use crate::engine::notation::Notation;
        let mut app = App::new();
        app.apply(Action::SetNotation(Notation::Auto));
        assert_eq!(app.state.notation, Notation::Auto);
    }

    // AC-6: precision input via C›p
    #[test]
    fn test_precision_input_flow() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        assert_eq!(app.mode, AppMode::PrecisionInput(String::new()));
        app.apply(Action::PrecisionDigit('6'));
        assert_eq!(app.mode, AppMode::PrecisionInput("6".to_string()));
        app.apply(Action::PrecisionSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.precision, 6);
        assert!(app.error_message.is_none());
    }

    // AC-7: precision out of range (0) → error, precision unchanged
    #[test]
    fn test_precision_out_of_range_rejected() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        app.apply(Action::PrecisionDigit('0'));
        app.apply(Action::PrecisionSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.precision, 15); // unchanged
        assert!(app.error_message.is_some());
    }

    // AC-6: precision input with 2 digits (e.g. "15")
    #[test]
    fn test_precision_input_two_digits() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        app.apply(Action::PrecisionDigit('1'));
        app.apply(Action::PrecisionDigit('5'));
        app.apply(Action::PrecisionSubmit);
        assert_eq!(app.state.precision, 15);
        assert!(app.error_message.is_none());
    }

    // AC-6: empty buffer Enter is no-op
    #[test]
    fn test_precision_submit_empty_is_noop() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        app.apply(Action::PrecisionSubmit);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.precision, 15); // unchanged
        assert!(app.error_message.is_none());
    }

    // AC-6: Esc cancels precision input
    #[test]
    fn test_precision_cancel() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        app.apply(Action::PrecisionDigit('5'));
        app.apply(Action::PrecisionCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.precision, 15); // unchanged
        assert!(app.error_message.is_none());
    }

    // AC-6: Backspace deletes last digit
    #[test]
    fn test_precision_backspace() {
        let mut app = App::new();
        app.apply(Action::EnterPrecisionInput);
        app.apply(Action::PrecisionDigit('1'));
        app.apply(Action::PrecisionDigit('2'));
        app.apply(Action::PrecisionBackspace);
        if let AppMode::PrecisionInput(buf) = &app.mode {
            assert_eq!(buf, "1", "backspace should delete last digit");
        } else {
            panic!("should still be in PrecisionInput mode");
        }
    }

    // AC-8: hex style error when base ≠ HEX
    #[test]
    fn test_hex_style_rejected_when_not_hex() {
        let mut app = App::new();
        // base is DEC by default
        app.apply(Action::SetHexStyle(HexStyle::Dollar));
        assert!(app.error_message.is_some(), "should error when base is not HEX");
        assert_eq!(app.state.hex_style, HexStyle::ZeroX); // unchanged
    }

    // AC-9: hex style accepted when base is HEX
    #[test]
    fn test_hex_style_accepted_when_hex() {
        let mut app = App::new();
        app.apply(Action::SetBase(Base::Hex));
        app.apply(Action::SetHexStyle(HexStyle::Dollar));
        assert!(app.error_message.is_none());
        assert_eq!(app.state.hex_style, HexStyle::Dollar);
    }

    // AC-10: Esc at chord level cancels (no change)
    #[test]
    fn test_config_chord_esc_cancels() {
        let mut app = App::new();
        app.apply(Action::EnterChordMode(ChordCategory::Config));
        app.apply(Action::ChordCancel);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.state.angle_mode, AngleMode::Deg); // unchanged
        assert!(app.error_message.is_none());
    }

    // AC-12: m and x are Noop in Normal mode
    #[test]
    fn test_m_and_x_are_noop() {
        use crate::input::handler::handle_key;
        use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
        fn key(code: KeyCode) -> KeyEvent {
            KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }
        }
        for c in ['m', 'x', 'X'] {
            assert_eq!(
                handle_key(&AppMode::Normal, key(KeyCode::Char(c))),
                Action::Noop,
                "'{}' should be Noop",
                c
            );
        }
    }

    // AC-11: notation and precision persist in CalcState (session serialization)
    #[test]
    fn test_notation_and_precision_in_calc_state() {
        use crate::engine::notation::Notation;
        let mut state = crate::engine::stack::CalcState::new();
        state.notation = Notation::Sci;
        state.precision = 6;
        let json = serde_json::to_string(&state).expect("serialize");
        let restored: crate::engine::stack::CalcState =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.notation, Notation::Sci);
        assert_eq!(restored.precision, 6);
    }
}
