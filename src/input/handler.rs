use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::engine::angle::AngleMode;
use crate::engine::base::{Base, HexStyle};
use crate::engine::notation::Notation;
use crate::engine::ops::Op;
use crate::input::{
    action::Action,
    mode::{AppMode, ChordCategory},
};

pub fn handle_key(mode: &AppMode, event: KeyEvent) -> Action {
    match mode {
        AppMode::Normal => match event.code {
            KeyCode::Char('Q') => Action::Quit,
            KeyCode::Char('q') => Action::Execute(Op::Square),
            KeyCode::Char('w') => Action::Execute(Op::Sqrt),
            KeyCode::Char('i') => Action::EnterAlphaMode,
            KeyCode::Char(c) if c.is_ascii_digit() => Action::InsertChar(c),
            KeyCode::Char('r') if event.modifiers.contains(KeyModifiers::CONTROL) => Action::Redo,
            KeyCode::Char('+') => Action::Execute(Op::Add),
            KeyCode::Char('-') => Action::Execute(Op::Sub),
            KeyCode::Char('*') => Action::Execute(Op::Mul),
            KeyCode::Char('/') => Action::Execute(Op::Div),
            KeyCode::Char('^') => Action::Execute(Op::Pow),
            KeyCode::Char('%') => Action::Execute(Op::Mod),
            KeyCode::Char('!') => Action::Execute(Op::Factorial),
            KeyCode::Char('s') => Action::Execute(Op::Swap),
            KeyCode::Char('d') => Action::Noop,
            KeyCode::Char('p') => Action::Execute(Op::Dup),
            KeyCode::Char('R') => Action::Execute(Op::Rotate),
            KeyCode::Char('r') => Action::EnterChordMode(ChordCategory::Rounding),
            KeyCode::Char('n') => Action::Execute(Op::Negate),
            KeyCode::Char('u') => Action::Undo,
            KeyCode::Char('y') => Action::Yank,
            KeyCode::Char('t') => Action::EnterChordMode(ChordCategory::Trig),
            KeyCode::Char('l') => Action::EnterChordMode(ChordCategory::Log),
            KeyCode::Char('f') => Action::EnterChordMode(ChordCategory::Functions),
            KeyCode::Char('c') => Action::EnterChordMode(ChordCategory::Constants),
            KeyCode::Char('C') => Action::EnterChordMode(ChordCategory::Config),
            KeyCode::Char('m') => Action::Noop,
            KeyCode::Char('x') => Action::Noop,
            KeyCode::Char('X') => Action::Noop,
            KeyCode::Char('S') => Action::EnterStoreMode,
            KeyCode::Backspace => Action::Execute(Op::Drop),
            KeyCode::Delete => Action::Execute(Op::Clear),
            KeyCode::Up => Action::EnterBrowseMode,
            KeyCode::Enter => Action::Execute(Op::Dup),
            KeyCode::Esc => Action::Noop,
            _ => Action::Noop,
        },
        AppMode::Chord(category) => match event.code {
            KeyCode::Esc => Action::ChordCancel,
            KeyCode::Char(c) => dispatch_chord_key(category, c),
            _ => Action::ChordInvalid,
        },
        AppMode::Insert(_) => match event.code {
            KeyCode::Enter => Action::InsertSubmit,
            KeyCode::Esc => Action::InsertCancel,
            KeyCode::Backspace => Action::InsertBackspace,
            KeyCode::Char('+') => Action::InsertSubmitThen(Op::Add),
            KeyCode::Char('-') => Action::InsertSubmitThen(Op::Sub),
            KeyCode::Char('*') => Action::InsertSubmitThen(Op::Mul),
            KeyCode::Char('/') => Action::InsertSubmitThen(Op::Div),
            KeyCode::Char('^') => Action::InsertSubmitThen(Op::Pow),
            KeyCode::Char('%') => Action::InsertSubmitThen(Op::Mod),
            KeyCode::Char('!') => Action::InsertSubmitThen(Op::Factorial),
            KeyCode::Char('n') => Action::InsertSubmitThen(Op::Negate),
            KeyCode::Char('q') => Action::InsertSubmitThen(Op::Square),
            KeyCode::Char('w') => Action::InsertSubmitThen(Op::Sqrt),
            KeyCode::Char('s') => Action::InsertSubmitThen(Op::Swap),
            KeyCode::Char('d') => Action::InsertSubmitThen(Op::Drop),
            KeyCode::Char('p') => Action::InsertSubmitThen(Op::Dup),
            KeyCode::Char('r') => Action::InsertSubmitThen(Op::Rotate),
            KeyCode::Char('R') => Action::InsertSubmitThen(Op::Rotate),
            KeyCode::Char(c) => Action::InsertChar(c),
            _ => Action::Noop,
        },
        AppMode::Alpha(_) => match event.code {
            KeyCode::Enter => Action::AlphaSubmit,
            KeyCode::Esc => Action::AlphaCancel,
            KeyCode::Backspace => Action::AlphaBackspace,
            KeyCode::Char(c) => Action::AlphaChar(c),
            _ => Action::Noop,
        },
        AppMode::AlphaStore(_) => match event.code {
            KeyCode::Enter => Action::AlphaSubmit,
            KeyCode::Esc => Action::AlphaCancel,
            KeyCode::Backspace => Action::AlphaBackspace,
            KeyCode::Char(c) => Action::AlphaChar(c),
            _ => Action::Noop,
        },
        AppMode::Browse(_) => match event.code {
            KeyCode::Up => Action::BrowseCursorUp,
            KeyCode::Down => Action::BrowseCursorDown,
            KeyCode::Enter => Action::BrowseConfirm,
            KeyCode::Esc => Action::BrowseCancel,
            _ => Action::Noop,
        },
        AppMode::PrecisionInput(_) => match event.code {
            KeyCode::Enter => Action::PrecisionSubmit,
            KeyCode::Esc => Action::PrecisionCancel,
            KeyCode::Backspace => Action::PrecisionBackspace,
            KeyCode::Char(c) if c.is_ascii_digit() => Action::PrecisionDigit(c),
            _ => Action::Noop,
        },
    }
}

/// Returns the human-readable label for a key event if it updates the last-command display,
/// or `None` if the key does not update the display (chord leaders, insert chars, navigation, etc.).
/// Must be called before `App::apply` so the mode still reflects the pre-action state.
pub fn command_label(mode: &AppMode, event: KeyEvent) -> Option<String> {
    match mode {
        AppMode::Normal => match event.code {
            KeyCode::Char('+') => Some(format!("+ → {}", op_name(Op::Add))),
            KeyCode::Char('-') => Some(format!("- → {}", op_name(Op::Sub))),
            KeyCode::Char('*') => Some(format!("* → {}", op_name(Op::Mul))),
            KeyCode::Char('/') => Some(format!("/ → {}", op_name(Op::Div))),
            KeyCode::Char('^') => Some(format!("^ → {}", op_name(Op::Pow))),
            KeyCode::Char('%') => Some(format!("% → {}", op_name(Op::Mod))),
            KeyCode::Char('!') => Some(format!("! → {}", op_name(Op::Factorial))),
            KeyCode::Char('s') => Some(format!("s → {}", op_name(Op::Swap))),
            KeyCode::Char('d') => Some(format!("d → {}", op_name(Op::Drop))),
            KeyCode::Char('p') => Some(format!("p → {}", op_name(Op::Dup))),
            KeyCode::Char('R') => Some(format!("R → {}", op_name(Op::Rotate))),
            KeyCode::Char('n') => Some(format!("n → {}", op_name(Op::Negate))),
            KeyCode::Char('q') => Some(format!("q → {}", op_name(Op::Square))),
            KeyCode::Char('w') => Some(format!("w → {}", op_name(Op::Sqrt))),
            KeyCode::Char('u') => Some("u → undo".to_string()),
            KeyCode::Char('r') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                Some("^r → redo".to_string())
            }
            KeyCode::Char('y') => Some("y → copy".to_string()),
            KeyCode::Enter => Some("↵ → dup".to_string()),
            // chord leaders, S, i, Q, Up: do not update label
            _ => None,
        },
        AppMode::Chord(category) => match event.code {
            KeyCode::Char(c) => {
                let leader = chord_leader_char(category);
                let label = chord_op_label(category, c)?;
                Some(format!("{}{} → {}", leader, c, label))
            }
            _ => None,
        },
        AppMode::Insert(_) => match event.code {
            KeyCode::Char('+') => Some(format!("+ → {}", op_name(Op::Add))),
            KeyCode::Char('-') => Some(format!("- → {}", op_name(Op::Sub))),
            KeyCode::Char('*') => Some(format!("* → {}", op_name(Op::Mul))),
            KeyCode::Char('/') => Some(format!("/ → {}", op_name(Op::Div))),
            KeyCode::Char('^') => Some(format!("^ → {}", op_name(Op::Pow))),
            KeyCode::Char('%') => Some(format!("% → {}", op_name(Op::Mod))),
            KeyCode::Char('!') => Some(format!("! → {}", op_name(Op::Factorial))),
            KeyCode::Char('n') => Some(format!("n → {}", op_name(Op::Negate))),
            KeyCode::Char('q') => Some(format!("q → {}", op_name(Op::Square))),
            KeyCode::Char('w') => Some(format!("w → {}", op_name(Op::Sqrt))),
            KeyCode::Char('s') => Some(format!("s → {}", op_name(Op::Swap))),
            KeyCode::Char('d') => Some(format!("d → {}", op_name(Op::Drop))),
            KeyCode::Char('p') => Some(format!("p → {}", op_name(Op::Dup))),
            KeyCode::Char('r') | KeyCode::Char('R') => Some(format!("r → {}", op_name(Op::Rotate))),
            // InsertChar, Enter (submit), Esc (cancel), Backspace: do not update label
            _ => None,
        },
        // Alpha, AlphaStore, Browse: no label updates
        _ => None,
    }
}

fn chord_leader_char(category: &ChordCategory) -> char {
    match category {
        ChordCategory::Trig => 't',
        ChordCategory::Log => 'l',
        ChordCategory::Functions => 'f',
        ChordCategory::Constants => 'c',
        ChordCategory::AngleMode => 'm',
        ChordCategory::Base => 'x',
        ChordCategory::HexStyle => 'X',
        ChordCategory::Rounding => 'r',
        ChordCategory::Config => 'C',
    }
}

fn chord_op_label(category: &ChordCategory, c: char) -> Option<&'static str> {
    match category {
        ChordCategory::Trig => match c {
            's' => Some("sin"),
            'c' => Some("cos"),
            'a' => Some("tan"),
            'S' => Some("asin"),
            'C' => Some("acos"),
            'A' => Some("atan"),
            _ => None,
        },
        ChordCategory::Log => match c {
            'l' => Some("ln"),
            'L' => Some("log₁₀"),
            'e' => Some("eˣ"),
            'E' => Some("10ˣ"),
            _ => None,
        },
        ChordCategory::Functions => match c {
            's' => Some("√x"),
            'q' => Some("x²"),
            'r' => Some("1/x"),
            'a' => Some("|x|"),
            _ => None,
        },
        ChordCategory::Constants => match c {
            'p' => Some("π"),
            'e' => Some("e"),
            'g' => Some("φ"),
            _ => None,
        },
        ChordCategory::AngleMode => match c {
            'd' => Some("deg"),
            'r' => Some("rad"),
            'g' => Some("grad"),
            _ => None,
        },
        ChordCategory::Base => match c {
            'c' => Some("dec"),
            'h' => Some("hex"),
            'o' => Some("oct"),
            'b' => Some("bin"),
            _ => None,
        },
        ChordCategory::HexStyle => match c {
            'c' => Some("0xFF"),
            'a' => Some("$FF"),
            's' => Some("#FF"),
            'i' => Some("FFh"),
            _ => None,
        },
        ChordCategory::Rounding => match c {
            'f' => Some("floor"),
            'c' => Some("ceil"),
            't' => Some("trunc"),
            'r' => Some("round"),
            's' => Some("sign"),
            _ => None,
        },
        ChordCategory::Config => match c {
            'd' => Some("deg"),
            'r' => Some("rad"),
            'g' => Some("grad"),
            'c' => Some("dec"),
            'h' => Some("hex"),
            'o' => Some("oct"),
            'b' => Some("bin"),
            'f' => Some("fixed"),
            's' => Some("sci"),
            'a' => Some("auto"),
            'p' => Some("prec"),
            '1' => Some("0xFF"),
            '2' => Some("$FF"),
            '3' => Some("#FF"),
            '4' => Some("FFh"),
            _ => None,
        },
    }
}

fn op_name(op: Op) -> &'static str {
    match op {
        Op::Add => "add",
        Op::Sub => "sub",
        Op::Mul => "mul",
        Op::Div => "div",
        Op::Pow => "pow",
        Op::Mod => "mod",
        Op::Negate => "neg",
        Op::Sqrt => "√x",
        Op::Square => "x²",
        Op::Reciprocal => "1/x",
        Op::Abs => "|x|",
        Op::Factorial => "fact",
        Op::Sin => "sin",
        Op::Cos => "cos",
        Op::Tan => "tan",
        Op::Asin => "asin",
        Op::Acos => "acos",
        Op::Atan => "atan",
        Op::Ln => "ln",
        Op::Log10 => "log₁₀",
        Op::Exp => "eˣ",
        Op::Exp10 => "10ˣ",
        Op::Swap => "swap",
        Op::Dup => "dup",
        Op::Drop => "drop",
        Op::Rotate => "rot",
        Op::Floor => "floor",
        Op::Ceil => "ceil",
        Op::Trunc => "trunc",
        Op::Round => "round",
        Op::Sign => "sign",
        Op::PushPi => "π",
        Op::PushE => "e",
        Op::PushPhi => "φ",
        _ => "?",
    }
}

fn dispatch_chord_key(category: &ChordCategory, c: char) -> Action {
    match category {
        ChordCategory::Trig => match c {
            's' => Action::Execute(Op::Sin),
            'c' => Action::Execute(Op::Cos),
            'a' => Action::Execute(Op::Tan),
            'S' => Action::Execute(Op::Asin),
            'C' => Action::Execute(Op::Acos),
            'A' => Action::Execute(Op::Atan),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Log => match c {
            'l' => Action::Execute(Op::Ln),
            'L' => Action::Execute(Op::Log10),
            'e' => Action::Execute(Op::Exp),
            'E' => Action::Execute(Op::Exp10),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Functions => match c {
            's' => Action::Execute(Op::Sqrt),
            'q' => Action::Execute(Op::Square),
            'r' => Action::Execute(Op::Reciprocal),
            'a' => Action::Execute(Op::Abs),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Constants => match c {
            'p' => Action::Execute(Op::PushPi),
            'e' => Action::Execute(Op::PushE),
            'g' => Action::Execute(Op::PushPhi),
            _ => Action::ChordInvalid,
        },
        ChordCategory::AngleMode => match c {
            'd' => Action::SetAngleMode(AngleMode::Deg),
            'r' => Action::SetAngleMode(AngleMode::Rad),
            'g' => Action::SetAngleMode(AngleMode::Grad),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Base => match c {
            'c' => Action::SetBase(Base::Dec),
            'h' => Action::SetBase(Base::Hex),
            'o' => Action::SetBase(Base::Oct),
            'b' => Action::SetBase(Base::Bin),
            _ => Action::ChordInvalid,
        },
        ChordCategory::HexStyle => match c {
            'c' => Action::SetHexStyle(HexStyle::ZeroX),
            'a' => Action::SetHexStyle(HexStyle::Dollar),
            's' => Action::SetHexStyle(HexStyle::Hash),
            'i' => Action::SetHexStyle(HexStyle::Suffix),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Rounding => match c {
            'f' => Action::Execute(Op::Floor),
            'c' => Action::Execute(Op::Ceil),
            't' => Action::Execute(Op::Trunc),
            'r' => Action::Execute(Op::Round),
            's' => Action::Execute(Op::Sign),
            _ => Action::ChordInvalid,
        },
        ChordCategory::Config => match c {
            'd' => Action::SetAngleMode(AngleMode::Deg),
            'r' => Action::SetAngleMode(AngleMode::Rad),
            'g' => Action::SetAngleMode(AngleMode::Grad),
            'c' => Action::SetBase(Base::Dec),
            'h' => Action::SetBase(Base::Hex),
            'o' => Action::SetBase(Base::Oct),
            'b' => Action::SetBase(Base::Bin),
            'f' => Action::SetNotation(Notation::Fixed),
            's' => Action::SetNotation(Notation::Sci),
            'a' => Action::SetNotation(Notation::Auto),
            'p' => Action::EnterPrecisionInput,
            '1' => Action::SetHexStyle(HexStyle::ZeroX),
            '2' => Action::SetHexStyle(HexStyle::Dollar),
            '3' => Action::SetHexStyle(HexStyle::Hash),
            '4' => Action::SetHexStyle(HexStyle::Suffix),
            _ => Action::ChordInvalid,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ops::Op;
    use crate::input::{
        action::Action,
        mode::{AppMode, ChordCategory},
    };
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn ctrl_key(c: char) -> KeyEvent {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    // AC 1: digit in Normal → InsertChar
    #[test]
    fn test_normal_digit_produces_insert_char() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('3'))),
            Action::InsertChar('3')
        );
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('0'))),
            Action::InsertChar('0')
        );
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('9'))),
            Action::InsertChar('9')
        );
    }

    // AC 2: 'i' in Normal → EnterAlphaMode (true Alpha mode)
    #[test]
    fn test_normal_i_enters_alpha_mode() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('i'))),
            Action::EnterAlphaMode
        );
    }

    // AC 5: Esc in Normal → Noop
    #[test]
    fn test_normal_esc_is_noop() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Esc)),
            Action::Noop
        );
    }

    // AC 8: Enter in Normal → Dup (HP convention)
    #[test]
    fn test_normal_enter_dups() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Enter)),
            Action::Execute(Op::Dup)
        );
    }

    // AC 7: all operation keys in Normal mode
    #[test]
    fn test_normal_ops() {
        let cases = [
            ('+', Action::Execute(Op::Add)),
            ('-', Action::Execute(Op::Sub)),
            ('*', Action::Execute(Op::Mul)),
            ('/', Action::Execute(Op::Div)),
            ('^', Action::Execute(Op::Pow)),
            ('%', Action::Execute(Op::Mod)),
            ('!', Action::Execute(Op::Factorial)),
            ('s', Action::Execute(Op::Swap)),
            ('p', Action::Execute(Op::Dup)),
            ('R', Action::Execute(Op::Rotate)),
            ('n', Action::Execute(Op::Negate)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Normal, key(KeyCode::Char(*c))),
                *expected,
                "key '{}' should produce {:?}",
                c,
                expected
            );
        }
    }

    // AC-8: d in Normal → Noop (use Backspace to drop)
    #[test]
    fn test_normal_d_is_noop() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('d'))),
            Action::Noop
        );
    }

    // Undo and Ctrl-R redo
    #[test]
    fn test_normal_undo_redo() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('u'))),
            Action::Undo
        );
        assert_eq!(handle_key(&AppMode::Normal, ctrl_key('r')), Action::Redo);
    }

    // AC-6: Backspace in Normal → Drop
    #[test]
    fn test_normal_backspace_is_drop() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Backspace)),
            Action::Execute(Op::Drop)
        );
    }

    // AC-7: Delete in Normal → Clear
    #[test]
    fn test_normal_delete_is_clear() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Delete)),
            Action::Execute(Op::Clear)
        );
    }

    // 'r' without Ctrl → enters Rounding chord (not Redo, not Rotate)
    #[test]
    fn test_normal_r_without_ctrl_is_rounding_chord() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('r'))),
            Action::EnterChordMode(ChordCategory::Rounding)
        );
    }

    // AC 1: chord leader keys in Normal mode
    #[test]
    fn test_normal_chord_leaders() {
        let cases = [
            ('t', Action::EnterChordMode(ChordCategory::Trig)),
            ('l', Action::EnterChordMode(ChordCategory::Log)),
            ('f', Action::EnterChordMode(ChordCategory::Functions)),
            ('c', Action::EnterChordMode(ChordCategory::Constants)),
            ('C', Action::EnterChordMode(ChordCategory::Config)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Normal, key(KeyCode::Char(*c))),
                *expected,
                "key '{}' should produce {:?}",
                c,
                expected
            );
        }
        // m, x, X are now Noop (removed in favour of C› config chord)
        for c in ['m', 'x', 'X'] {
            assert_eq!(
                handle_key(&AppMode::Normal, key(KeyCode::Char(c))),
                Action::Noop,
                "key '{}' should be Noop after config chord rebinding",
                c
            );
        }
    }

    // AC 2: Trig chord second keys
    #[test]
    fn test_chord_trig_second_keys() {
        let cases = [
            ('s', Action::Execute(Op::Sin)),
            ('c', Action::Execute(Op::Cos)),
            ('a', Action::Execute(Op::Tan)),
            ('S', Action::Execute(Op::Asin)),
            ('C', Action::Execute(Op::Acos)),
            ('A', Action::Execute(Op::Atan)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Trig), key(KeyCode::Char(*c))),
                *expected,
                "trig key '{}' should produce {:?}",
                c,
                expected
            );
        }
    }

    // AC 2: Log chord second keys
    #[test]
    fn test_chord_log_second_keys() {
        let cases = [
            ('l', Action::Execute(Op::Ln)),
            ('L', Action::Execute(Op::Log10)),
            ('e', Action::Execute(Op::Exp)),
            ('E', Action::Execute(Op::Exp10)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Log), key(KeyCode::Char(*c))),
                *expected
            );
        }
    }

    // AC 2: Functions chord second keys
    #[test]
    fn test_chord_fn_second_keys() {
        let cases = [
            ('s', Action::Execute(Op::Sqrt)),
            ('q', Action::Execute(Op::Square)),
            ('r', Action::Execute(Op::Reciprocal)),
            ('a', Action::Execute(Op::Abs)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(
                    &AppMode::Chord(ChordCategory::Functions),
                    key(KeyCode::Char(*c))
                ),
                *expected
            );
        }
    }

    // AC 2: Constants chord second keys
    #[test]
    fn test_chord_const_second_keys() {
        use crate::engine::ops::Op;
        let cases = [
            ('p', Action::Execute(Op::PushPi)),
            ('e', Action::Execute(Op::PushE)),
            ('g', Action::Execute(Op::PushPhi)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(
                    &AppMode::Chord(ChordCategory::Constants),
                    key(KeyCode::Char(*c))
                ),
                *expected
            );
        }
    }

    // AC 2: Angle mode chord second keys
    #[test]
    fn test_chord_angle_second_keys() {
        use crate::engine::angle::AngleMode;
        let cases = [
            ('d', Action::SetAngleMode(AngleMode::Deg)),
            ('r', Action::SetAngleMode(AngleMode::Rad)),
            ('g', Action::SetAngleMode(AngleMode::Grad)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(
                    &AppMode::Chord(ChordCategory::AngleMode),
                    key(KeyCode::Char(*c))
                ),
                *expected
            );
        }
    }

    // AC 2: Base chord second keys
    #[test]
    fn test_chord_base_second_keys() {
        use crate::engine::base::Base;
        let cases = [
            ('c', Action::SetBase(Base::Dec)),
            ('h', Action::SetBase(Base::Hex)),
            ('o', Action::SetBase(Base::Oct)),
            ('b', Action::SetBase(Base::Bin)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Base), key(KeyCode::Char(*c))),
                *expected
            );
        }
    }

    // AC 2: HexStyle chord second keys
    #[test]
    fn test_chord_hex_style_second_keys() {
        use crate::engine::base::HexStyle;
        let cases = [
            ('c', Action::SetHexStyle(HexStyle::ZeroX)),
            ('a', Action::SetHexStyle(HexStyle::Dollar)),
            ('s', Action::SetHexStyle(HexStyle::Hash)),
            ('i', Action::SetHexStyle(HexStyle::Suffix)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(
                    &AppMode::Chord(ChordCategory::HexStyle),
                    key(KeyCode::Char(*c))
                ),
                *expected
            );
        }
    }

    // AC 3: Esc in chord mode → ChordCancel
    #[test]
    fn test_chord_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Trig), key(KeyCode::Esc)),
            Action::ChordCancel
        );
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Base), key(KeyCode::Esc)),
            Action::ChordCancel
        );
    }

    // AC 4: Unknown char in chord mode → ChordInvalid
    #[test]
    fn test_chord_invalid_key() {
        assert_eq!(
            handle_key(
                &AppMode::Chord(ChordCategory::Trig),
                key(KeyCode::Char('z'))
            ),
            Action::ChordInvalid
        );
        assert_eq!(
            handle_key(
                &AppMode::Chord(ChordCategory::Base),
                key(KeyCode::Char('z'))
            ),
            Action::ChordInvalid
        );
    }

    // Insert mode: printable char → InsertChar
    #[test]
    fn test_insert_char_appends() {
        assert_eq!(
            handle_key(&AppMode::Insert("4".into()), key(KeyCode::Char('2'))),
            Action::InsertChar('2')
        );
    }

    // Insert mode: Enter → InsertSubmit
    #[test]
    fn test_insert_enter_submits() {
        assert_eq!(
            handle_key(&AppMode::Insert("42".into()), key(KeyCode::Enter)),
            Action::InsertSubmit
        );
    }

    // Insert mode: Esc → InsertCancel
    #[test]
    fn test_insert_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::Insert("42".into()), key(KeyCode::Esc)),
            Action::InsertCancel
        );
    }

    // Insert mode: unknown key → Noop
    #[test]
    fn test_insert_unknown_key_is_noop() {
        assert_eq!(
            handle_key(&AppMode::Insert("".into()), key(KeyCode::F(1))),
            Action::Noop
        );
    }

    // Insert mode: op shortcut keys trigger InsertSubmitThen
    #[test]
    fn test_insert_op_shortcuts() {
        let cases = [
            ('+', Action::InsertSubmitThen(Op::Add)),
            ('-', Action::InsertSubmitThen(Op::Sub)),
            ('*', Action::InsertSubmitThen(Op::Mul)),
            ('/', Action::InsertSubmitThen(Op::Div)),
            ('^', Action::InsertSubmitThen(Op::Pow)),
            ('%', Action::InsertSubmitThen(Op::Mod)),
            ('!', Action::InsertSubmitThen(Op::Factorial)),
            ('s', Action::InsertSubmitThen(Op::Swap)),
            ('d', Action::InsertSubmitThen(Op::Drop)),
            ('p', Action::InsertSubmitThen(Op::Dup)),
            ('r', Action::InsertSubmitThen(Op::Rotate)),
            ('n', Action::InsertSubmitThen(Op::Negate)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Insert("3".into()), key(KeyCode::Char(*c))),
                *expected,
                "Insert mode key '{}' should be InsertSubmitThen",
                c
            );
        }
    }

    // Alpha mode: ALL chars (including shortcut keys) → AlphaChar
    #[test]
    fn test_alpha_mode_all_chars_literal() {
        let shortcut_chars = ['+', '-', '*', '/', '^', '%', '!', 's', 'd', 'p', 'r', 'n'];
        for c in &shortcut_chars {
            assert_eq!(
                handle_key(&AppMode::Alpha("".into()), key(KeyCode::Char(*c))),
                Action::AlphaChar(*c),
                "Alpha mode key '{}' should be literal AlphaChar, not a shortcut",
                c
            );
        }
        assert_eq!(
            handle_key(&AppMode::Alpha("r1".into()), key(KeyCode::Char('r'))),
            Action::AlphaChar('r'),
            "'r' in Alpha mode should be literal, not Rotate"
        );
    }

    // Alpha mode: Enter → AlphaSubmit
    #[test]
    fn test_alpha_enter_submits() {
        assert_eq!(
            handle_key(&AppMode::Alpha("r1 RCL".into()), key(KeyCode::Enter)),
            Action::AlphaSubmit
        );
    }

    // Alpha mode: Esc → AlphaCancel
    #[test]
    fn test_alpha_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::Alpha("r1".into()), key(KeyCode::Esc)),
            Action::AlphaCancel
        );
    }

    // Alpha mode: unknown key → Noop
    #[test]
    fn test_alpha_unknown_key_is_noop() {
        assert_eq!(
            handle_key(&AppMode::Alpha("".into()), key(KeyCode::F(1))),
            Action::Noop
        );
    }

    // AC 1: 'S' in Normal → EnterStoreMode
    #[test]
    fn test_normal_s_enters_store_mode() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('S'))),
            Action::EnterStoreMode
        );
    }

    // AlphaStore: Enter → AlphaSubmit
    #[test]
    fn test_alpha_store_enter_submits() {
        assert_eq!(
            handle_key(&AppMode::AlphaStore("myvar".into()), key(KeyCode::Enter)),
            Action::AlphaSubmit
        );
    }

    // AlphaStore: Esc → AlphaCancel
    #[test]
    fn test_alpha_store_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::AlphaStore("".into()), key(KeyCode::Esc)),
            Action::AlphaCancel
        );
    }

    // AlphaStore: Backspace → AlphaBackspace
    #[test]
    fn test_alpha_store_backspace() {
        assert_eq!(
            handle_key(&AppMode::AlphaStore("x".into()), key(KeyCode::Backspace)),
            Action::AlphaBackspace
        );
    }

    // AlphaStore: printable char → AlphaChar
    #[test]
    fn test_alpha_store_char_appends() {
        assert_eq!(
            handle_key(&AppMode::AlphaStore("".into()), key(KeyCode::Char('m'))),
            Action::AlphaChar('m')
        );
    }

    // AlphaStore: unknown key → Noop
    #[test]
    fn test_alpha_store_unknown_key_is_noop() {
        assert_eq!(
            handle_key(&AppMode::AlphaStore("".into()), key(KeyCode::F(1))),
            Action::Noop
        );
    }

    // AC-1: 'q' in Normal → Square (x²)
    #[test]
    fn test_normal_q_squares() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('q'))),
            Action::Execute(Op::Square)
        );
    }

    // AC-2: 'w' in Normal → Sqrt (√)
    #[test]
    fn test_normal_w_sqrts() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('w'))),
            Action::Execute(Op::Sqrt)
        );
    }

    // 'Q' (shift) in Normal → Quit (quit still accessible)
    #[test]
    fn test_normal_shift_q_quits() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('Q'))),
            Action::Quit
        );
    }

    // 'q' no longer quits
    #[test]
    fn test_normal_q_does_not_quit() {
        assert_ne!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('q'))),
            Action::Quit
        );
    }

    // AC-4: f› chord still dispatches Square and Sqrt
    #[test]
    fn test_chord_fn_q_still_squares() {
        assert_eq!(
            handle_key(
                &AppMode::Chord(ChordCategory::Functions),
                key(KeyCode::Char('q'))
            ),
            Action::Execute(Op::Square)
        );
    }

    #[test]
    fn test_chord_fn_s_still_sqrts() {
        assert_eq!(
            handle_key(
                &AppMode::Chord(ChordCategory::Functions),
                key(KeyCode::Char('s'))
            ),
            Action::Execute(Op::Sqrt)
        );
    }

    // Insert mode: 'q' → InsertSubmitThen(Square)
    #[test]
    fn test_insert_q_submit_then_square() {
        assert_eq!(
            handle_key(&AppMode::Insert("3".into()), key(KeyCode::Char('q'))),
            Action::InsertSubmitThen(Op::Square)
        );
    }

    // Insert mode: 'w' → InsertSubmitThen(Sqrt)
    #[test]
    fn test_insert_w_submit_then_sqrt() {
        assert_eq!(
            handle_key(&AppMode::Insert("9".into()), key(KeyCode::Char('w'))),
            Action::InsertSubmitThen(Op::Sqrt)
        );
    }

    // Browse mode entry: ↑ in Normal → EnterBrowseMode
    #[test]
    fn test_normal_up_enters_browse_mode() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Up)),
            Action::EnterBrowseMode
        );
    }

    // AC-3: ↑ in Browse → BrowseCursorUp
    #[test]
    fn test_browse_up_is_cursor_up() {
        assert_eq!(
            handle_key(&AppMode::Browse(2), key(KeyCode::Up)),
            Action::BrowseCursorUp
        );
    }

    // AC-4: ↓ in Browse → BrowseCursorDown
    #[test]
    fn test_browse_down_is_cursor_down() {
        assert_eq!(
            handle_key(&AppMode::Browse(3), key(KeyCode::Down)),
            Action::BrowseCursorDown
        );
    }

    // AC-1: Enter in Browse → BrowseConfirm
    #[test]
    fn test_browse_enter_confirms() {
        assert_eq!(
            handle_key(&AppMode::Browse(2), key(KeyCode::Enter)),
            Action::BrowseConfirm
        );
    }

    // AC-2: Esc in Browse → BrowseCancel
    #[test]
    fn test_browse_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::Browse(2), key(KeyCode::Esc)),
            Action::BrowseCancel
        );
    }

    // AC-11: unrecognised keys in Browse → Noop (silently consumed)
    #[test]
    fn test_browse_unrecognised_keys_are_noop() {
        let unknown_keys = [
            KeyCode::Char('a'),
            KeyCode::Char('+'),
            KeyCode::F(1),
            KeyCode::Backspace,
        ];
        for code in &unknown_keys {
            assert_eq!(
                handle_key(&AppMode::Browse(2), key(*code)),
                Action::Noop,
                "key {:?} in Browse mode should be Noop",
                code
            );
        }
    }

    // ── Rounding chord (apply-rounding-and-sign-ops) ─────────────────────────

    // r in Normal → enters Rounding chord
    #[test]
    fn test_normal_r_enters_rounding_chord() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('r'))),
            Action::EnterChordMode(ChordCategory::Rounding)
        );
    }

    // R in Normal → Rotate (rebind)
    #[test]
    fn test_normal_shift_r_rotates() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('R'))),
            Action::Execute(Op::Rotate)
        );
    }

    // AC-14: Esc in Rounding chord → ChordCancel
    #[test]
    fn test_rounding_chord_esc_cancels() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Esc)),
            Action::ChordCancel
        );
    }

    // Rounding chord second keys dispatch correctly
    #[test]
    fn test_rounding_chord_floor() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('f'))),
            Action::Execute(Op::Floor)
        );
    }

    #[test]
    fn test_rounding_chord_ceil() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('c'))),
            Action::Execute(Op::Ceil)
        );
    }

    #[test]
    fn test_rounding_chord_trunc() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('t'))),
            Action::Execute(Op::Trunc)
        );
    }

    #[test]
    fn test_rounding_chord_round() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('r'))),
            Action::Execute(Op::Round)
        );
    }

    #[test]
    fn test_rounding_chord_sign() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('s'))),
            Action::Execute(Op::Sign)
        );
    }

    // AC-1: single-key op returns label
    #[test]
    fn test_command_label_single_op() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('+')));
        assert_eq!(label.as_deref(), Some("+ → add"));
    }

    // AC-2: chord second key returns two-key label
    #[test]
    fn test_command_label_chord_two_keys() {
        let label = command_label(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('f')));
        assert_eq!(label.as_deref(), Some("rf → floor"));
    }

    // AC-2: mode-change chord returns label
    #[test]
    fn test_command_label_chord_mode_change() {
        let label = command_label(&AppMode::Chord(ChordCategory::AngleMode), key(KeyCode::Char('d')));
        assert_eq!(label.as_deref(), Some("md → deg"));
    }

    // AC-3: label returned regardless of stack state (key event only, not result)
    #[test]
    fn test_command_label_returned_for_any_key() {
        // command_label checks the key, not whether the op will succeed
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('+')));
        assert!(label.is_some(), "label returned even if stack would be empty");
    }

    // AC-4: navigation keys do not return a label
    #[test]
    fn test_command_label_navigation_returns_none() {
        assert_eq!(command_label(&AppMode::Normal, key(KeyCode::Up)), None);
        assert_eq!(command_label(&AppMode::Browse(2), key(KeyCode::Enter)), None);
        assert_eq!(command_label(&AppMode::Browse(2), key(KeyCode::Up)), None);
    }

    // AC-5: undo returns label
    #[test]
    fn test_command_label_undo() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('u')));
        assert_eq!(label.as_deref(), Some("u → undo"));
    }

    // redo (ctrl-r) returns label with ^ prefix
    #[test]
    fn test_command_label_redo() {
        let label = command_label(&AppMode::Normal, ctrl_key('r'));
        assert_eq!(label.as_deref(), Some("^r → redo"));
    }

    // AC-7: InsertSubmitThen op key returns label
    #[test]
    fn test_command_label_insert_submit_then() {
        let label = command_label(&AppMode::Insert(String::new()), key(KeyCode::Char('+')));
        assert_eq!(label.as_deref(), Some("+ → add"));
    }

    // AC-9: yank returns copy label
    #[test]
    fn test_command_label_yank() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('y')));
        assert_eq!(label.as_deref(), Some("y → copy"));
    }

    // Enter in Normal mode → dup
    #[test]
    fn test_command_label_enter_normal_dup() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Enter));
        assert_eq!(label.as_deref(), Some("↵ → dup"));
    }

    // AC-11: EnterStoreMode (S) does not return a label
    #[test]
    fn test_command_label_enter_store_mode_none() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('S')));
        assert_eq!(label, None, "EnterStoreMode should not update label");
    }

    // Chord leader key (r in Normal) does not return a label
    #[test]
    fn test_command_label_chord_leader_none() {
        let label = command_label(&AppMode::Normal, key(KeyCode::Char('r')));
        assert_eq!(label, None, "chord leader should not update label");
    }

    // Invalid chord key returns None (ChordInvalid does not update label)
    #[test]
    fn test_command_label_invalid_chord_none() {
        let label = command_label(&AppMode::Chord(ChordCategory::Rounding), key(KeyCode::Char('z')));
        assert_eq!(label, None);
    }

    // ── configure-settings-chord handler tests ──────────────────────────────

    // C in Normal → EnterChordMode(Config)
    #[test]
    fn test_normal_c_enters_config_chord() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('C'))),
            Action::EnterChordMode(ChordCategory::Config)
        );
    }

    // Config chord angle mode keys
    #[test]
    fn test_config_chord_angle_keys() {
        let cases = [
            ('d', Action::SetAngleMode(AngleMode::Deg)),
            ('r', Action::SetAngleMode(AngleMode::Rad)),
            ('g', Action::SetAngleMode(AngleMode::Grad)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char(*c))),
                *expected,
                "config chord '{}' should set angle mode",
                c
            );
        }
    }

    // Config chord base keys
    #[test]
    fn test_config_chord_base_keys() {
        let cases = [
            ('c', Action::SetBase(Base::Dec)),
            ('h', Action::SetBase(Base::Hex)),
            ('o', Action::SetBase(Base::Oct)),
            ('b', Action::SetBase(Base::Bin)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char(*c))),
                *expected,
                "config chord '{}' should set base",
                c
            );
        }
    }

    // Config chord notation keys
    #[test]
    fn test_config_chord_notation_keys() {
        let cases = [
            ('f', Action::SetNotation(Notation::Fixed)),
            ('s', Action::SetNotation(Notation::Sci)),
            ('a', Action::SetNotation(Notation::Auto)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char(*c))),
                *expected,
                "config chord '{}' should set notation",
                c
            );
        }
    }

    // Config chord 'p' → EnterPrecisionInput
    #[test]
    fn test_config_chord_p_enters_precision() {
        assert_eq!(
            handle_key(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char('p'))),
            Action::EnterPrecisionInput
        );
    }

    // Config chord hex style keys (1–4)
    #[test]
    fn test_config_chord_hex_style_keys() {
        let cases = [
            ('1', Action::SetHexStyle(HexStyle::ZeroX)),
            ('2', Action::SetHexStyle(HexStyle::Dollar)),
            ('3', Action::SetHexStyle(HexStyle::Hash)),
            ('4', Action::SetHexStyle(HexStyle::Suffix)),
        ];
        for (c, expected) in &cases {
            assert_eq!(
                handle_key(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char(*c))),
                *expected,
                "config chord '{}' should set hex style",
                c
            );
        }
    }

    // PrecisionInput mode: digits → PrecisionDigit, Enter → Submit, Esc → Cancel, Backspace → Backspace
    #[test]
    fn test_precision_input_mode_keys() {
        let mode = AppMode::PrecisionInput(String::new());
        assert_eq!(handle_key(&mode, key(KeyCode::Char('5'))), Action::PrecisionDigit('5'));
        assert_eq!(handle_key(&mode, key(KeyCode::Enter)), Action::PrecisionSubmit);
        assert_eq!(handle_key(&mode, key(KeyCode::Esc)), Action::PrecisionCancel);
        assert_eq!(handle_key(&mode, key(KeyCode::Backspace)), Action::PrecisionBackspace);
        // non-digit chars are Noop in PrecisionInput
        assert_eq!(handle_key(&mode, key(KeyCode::Char('a'))), Action::Noop);
    }

    // command_label for Config chord
    #[test]
    fn test_command_label_config_chord() {
        let label = command_label(&AppMode::Chord(ChordCategory::Config), key(KeyCode::Char('s')));
        assert_eq!(label.as_deref(), Some("Cs → sci"));
    }
}
