use crate::engine::angle::AngleMode;
use crate::engine::base::{Base, HexStyle};
use crate::engine::notation::Notation;
use crate::engine::ops::Op;
use crate::engine::value::CalcValue;
use crate::input::mode::ChordCategory;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    #[allow(dead_code)] // constructed in tests and future session-restore path
    Push(CalcValue),
    Execute(Op),
    SetBase(Base),
    SetAngleMode(AngleMode),
    SetHexStyle(HexStyle),
    SetNotation(Notation),
    EnterPrecisionInput,
    PrecisionDigit(char),
    PrecisionBackspace,
    PrecisionSubmit,
    PrecisionCancel,
    StoreRegister(String),
    RecallRegister(String),
    DeleteRegister(String),
    ResetSession,
    EnterStoreMode,
    Undo,
    Redo,
    Yank,
    EnterAlphaMode,
    InsertChar(char),
    InsertBackspace,
    InsertSubmit,
    InsertSubmitThen(Op),
    InsertCancel,
    AlphaChar(char),
    AlphaBackspace,
    AlphaSubmit,
    AlphaCancel,
    EnterChordMode(ChordCategory),
    ChordCancel,
    ChordInvalid,
    EnterBrowseMode,
    BrowseCursorUp,
    BrowseCursorDown,
    BrowseConfirm,
    BrowseCancel,
    Quit,
    Noop,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dashu::integer::IBig;

    #[test]
    fn test_action_constructible() {
        let _ = Action::Push(CalcValue::Integer(IBig::from(42)));
        let _ = Action::Execute(Op::Sin);
        let _ = Action::Undo;
        let _ = Action::ResetSession;
    }
}
