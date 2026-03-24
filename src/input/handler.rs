use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::engine::angle::AngleMode;
use crate::engine::base::{Base, HexStyle};
use crate::engine::ops::Op;
use crate::input::{
    action::Action,
    mode::{AppMode, ChordCategory},
};

pub fn handle_key(mode: &AppMode, event: KeyEvent) -> Action {
    match mode {
        AppMode::Normal => match event.code {
            KeyCode::Char('q') => Action::Quit,
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
            KeyCode::Char('d') => Action::Execute(Op::Drop),
            KeyCode::Char('p') => Action::Execute(Op::Dup),
            KeyCode::Char('r') => Action::Execute(Op::Rotate),
            KeyCode::Char('n') => Action::Execute(Op::Negate),
            KeyCode::Char('u') => Action::Undo,
            KeyCode::Char('y') => Action::Yank,
            KeyCode::Char('t') => Action::EnterChordMode(ChordCategory::Trig),
            KeyCode::Char('l') => Action::EnterChordMode(ChordCategory::Log),
            KeyCode::Char('f') => Action::EnterChordMode(ChordCategory::Functions),
            KeyCode::Char('c') => Action::EnterChordMode(ChordCategory::Constants),
            KeyCode::Char('m') => Action::EnterChordMode(ChordCategory::AngleMode),
            KeyCode::Char('x') => Action::EnterChordMode(ChordCategory::Base),
            KeyCode::Char('X') => Action::EnterChordMode(ChordCategory::HexStyle),
            KeyCode::Char('S') => Action::EnterStoreMode,
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
            KeyCode::Char('s') => Action::InsertSubmitThen(Op::Swap),
            KeyCode::Char('d') => Action::InsertSubmitThen(Op::Drop),
            KeyCode::Char('p') => Action::InsertSubmitThen(Op::Dup),
            KeyCode::Char('r') => Action::InsertSubmitThen(Op::Rotate),
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
            ('d', Action::Execute(Op::Drop)),
            ('p', Action::Execute(Op::Dup)),
            ('r', Action::Execute(Op::Rotate)),
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

    // Undo and Ctrl-R redo
    #[test]
    fn test_normal_undo_redo() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('u'))),
            Action::Undo
        );
        assert_eq!(handle_key(&AppMode::Normal, ctrl_key('r')), Action::Redo);
    }

    // 'r' without Ctrl → Rotate (not Redo)
    #[test]
    fn test_normal_r_without_ctrl_is_rotate() {
        assert_eq!(
            handle_key(&AppMode::Normal, key(KeyCode::Char('r'))),
            Action::Execute(Op::Rotate)
        );
    }

    // AC 1: All 7 chord leader keys in Normal mode
    #[test]
    fn test_normal_chord_leaders() {
        let cases = [
            ('t', Action::EnterChordMode(ChordCategory::Trig)),
            ('l', Action::EnterChordMode(ChordCategory::Log)),
            ('f', Action::EnterChordMode(ChordCategory::Functions)),
            ('c', Action::EnterChordMode(ChordCategory::Constants)),
            ('m', Action::EnterChordMode(ChordCategory::AngleMode)),
            ('x', Action::EnterChordMode(ChordCategory::Base)),
            ('X', Action::EnterChordMode(ChordCategory::HexStyle)),
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
}
