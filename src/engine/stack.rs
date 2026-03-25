use crate::engine::angle::AngleMode;
use crate::engine::base::{Base, HexStyle};
use crate::engine::error::CalcError;
use crate::engine::notation::Notation;
use crate::engine::value::CalcValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalcState {
    pub stack: Vec<CalcValue>,
    pub registers: HashMap<String, CalcValue>,
    pub angle_mode: AngleMode,
    pub base: Base,
    pub hex_style: HexStyle,
    #[serde(default)]
    pub notation: Notation,
    #[serde(default = "default_precision")]
    pub precision: usize,
}

fn default_precision() -> usize {
    15
}

impl CalcState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            registers: HashMap::new(),
            angle_mode: AngleMode::Deg,
            base: Base::Dec,
            hex_style: HexStyle::ZeroX,
            notation: Notation::Fixed,
            precision: 15,
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&CalcValue> {
        self.stack.last()
    }

    pub fn push(&mut self, val: CalcValue) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Result<CalcValue, CalcError> {
        self.stack.pop().ok_or(CalcError::StackUnderflow)
    }

    pub fn swap(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 2 {
            return Err(CalcError::StackUnderflow);
        }
        let n = self.stack.len();
        self.stack.swap(n - 1, n - 2);
        Ok(())
    }

    pub fn dup(&mut self) -> Result<(), CalcError> {
        let top = self.stack.last().ok_or(CalcError::StackUnderflow)?.clone();
        self.stack.push(top);
        Ok(())
    }

    pub fn drop(&mut self) -> Result<(), CalcError> {
        self.pop().map(|_| ())
    }

    pub fn rotate(&mut self) -> Result<(), CalcError> {
        if self.stack.len() < 3 {
            return Err(CalcError::StackUnderflow);
        }
        let n = self.stack.len();
        let x = self.stack[n - 1].clone(); // old top (X)
        let y = self.stack[n - 2].clone(); // old second (Y)
        let z = self.stack[n - 3].clone(); // old third (Z)
        self.stack[n - 3] = x; // old top → Z position
        self.stack[n - 2] = z; // old Z → Y position
        self.stack[n - 1] = y; // old Y → new top (X)
        Ok(())
    }

    /// Roll the item at 1-indexed position `n` from the top to the top.
    /// Position 1 = top (no-op equivalent); position 2 = swap equivalent.
    /// Requires n ≥ 2 and stack depth ≥ 2.
    pub fn roll(&mut self, n: usize) -> Result<(), CalcError> {
        if self.stack.len() < 2 || n < 2 || n > self.stack.len() {
            return Err(CalcError::StackUnderflow);
        }
        let idx = self.stack.len() - n;
        let val = self.stack.remove(idx);
        self.stack.push(val);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

impl Default for CalcState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{error::CalcError, value::CalcValue};
    use dashu::integer::IBig;

    fn int(n: i32) -> CalcValue {
        CalcValue::Integer(IBig::from(n))
    }

    // ── Story 1.2 baseline tests ────────────────────────────────────────────

    #[test]
    fn test_calcstate_new_defaults() {
        let state = CalcState::new();
        assert_eq!(state.angle_mode, AngleMode::Deg);
        assert_eq!(state.base, Base::Dec);
        assert_eq!(state.hex_style, HexStyle::ZeroX);
        assert!(state.registers.is_empty());
    }

    #[test]
    fn test_calcstate_is_empty() {
        assert!(CalcState::new().is_empty());
    }

    #[test]
    fn test_calcstate_depth_zero() {
        assert_eq!(CalcState::new().depth(), 0);
    }

    // ── push ────────────────────────────────────────────────────────────────

    #[test]
    fn test_push_adds_to_top() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        assert_eq!(s.depth(), 2);
        assert_eq!(s.peek(), Some(&int(2)));
    }

    #[test]
    fn test_push_preserves_existing_items() {
        let mut s = CalcState::new();
        s.push(int(7));
        s.push(int(99));
        // bottom item should still be 7
        assert_eq!(s.stack[0], int(7));
        assert_eq!(s.stack[1], int(99));
    }

    // ── pop ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_pop_returns_top_and_removes_it() {
        let mut s = CalcState::new();
        s.push(int(3));
        s.push(int(5));
        let val = s.pop().unwrap();
        assert_eq!(val, int(5));
        assert_eq!(s.depth(), 1);
        assert_eq!(s.peek(), Some(&int(3)));
    }

    #[test]
    fn test_pop_on_empty_is_stack_underflow() {
        let mut s = CalcState::new();
        assert!(matches!(s.pop(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 0); // unchanged
    }

    // ── swap ────────────────────────────────────────────────────────────────

    #[test]
    fn test_swap_exchanges_top_two() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        s.swap().unwrap();
        assert_eq!(s.peek(), Some(&int(1)));
        assert_eq!(s.stack[0], int(2));
        assert_eq!(s.depth(), 2);
    }

    #[test]
    fn test_swap_does_not_affect_deeper_items() {
        let mut s = CalcState::new();
        s.push(int(10));
        s.push(int(20));
        s.push(int(30));
        s.swap().unwrap();
        assert_eq!(s.stack[0], int(10)); // untouched
        assert_eq!(s.stack[1], int(30));
        assert_eq!(s.stack[2], int(20));
    }

    #[test]
    fn test_swap_underflow_with_one_item() {
        let mut s = CalcState::new();
        s.push(int(1));
        assert!(matches!(s.swap(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 1); // unchanged
        assert_eq!(s.peek(), Some(&int(1)));
    }

    #[test]
    fn test_swap_underflow_on_empty() {
        let mut s = CalcState::new();
        assert!(matches!(s.swap(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 0);
    }

    // ── dup ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_dup_duplicates_top() {
        let mut s = CalcState::new();
        s.push(int(42));
        s.dup().unwrap();
        assert_eq!(s.depth(), 2);
        assert_eq!(s.peek(), Some(&int(42)));
        assert_eq!(s.stack[0], int(42));
    }

    #[test]
    fn test_dup_underflow_on_empty() {
        let mut s = CalcState::new();
        assert!(matches!(s.dup(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 0);
    }

    // ── drop ────────────────────────────────────────────────────────────────

    #[test]
    fn test_drop_removes_top() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        s.drop().unwrap();
        assert_eq!(s.depth(), 1);
        assert_eq!(s.peek(), Some(&int(1)));
    }

    #[test]
    fn test_drop_underflow_on_empty() {
        let mut s = CalcState::new();
        assert!(matches!(s.drop(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 0);
    }

    // ── rotate ──────────────────────────────────────────────────────────────

    #[test]
    fn test_rotate_roll_down_correct_direction() {
        // Stack (bottom→top): [1, 2, 3] — Z=1, Y=2, X=3 (top)
        // After rotate: X(3)→Z, Y(2)→new top, Z(1)→Y
        // Expected vec: [3, 1, 2] — new top = 2
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        s.push(int(3));
        s.rotate().unwrap();
        assert_eq!(s.depth(), 3);
        assert_eq!(s.stack[0], int(3)); // old X → new Z
        assert_eq!(s.stack[1], int(1)); // old Z → new Y
        assert_eq!(s.stack[2], int(2)); // old Y → new X (top)
        assert_eq!(s.peek(), Some(&int(2)));
    }

    #[test]
    fn test_rotate_does_not_affect_items_below_top_three() {
        let mut s = CalcState::new();
        s.push(int(99)); // deep item — must be untouched
        s.push(int(1));
        s.push(int(2));
        s.push(int(3));
        s.rotate().unwrap();
        assert_eq!(s.stack[0], int(99)); // untouched
        assert_eq!(s.depth(), 4);
    }

    #[test]
    fn test_rotate_underflow_with_two_items() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        assert!(matches!(s.rotate(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 2); // unchanged
        assert_eq!(s.peek(), Some(&int(2)));
    }

    #[test]
    fn test_rotate_underflow_with_one_item() {
        let mut s = CalcState::new();
        s.push(int(1));
        assert!(matches!(s.rotate(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 1);
    }

    #[test]
    fn test_rotate_underflow_on_empty() {
        let mut s = CalcState::new();
        assert!(matches!(s.rotate(), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 0);
    }

    // ── clear ───────────────────────────────────────────────────────────────

    #[test]
    fn test_clear_empties_stack() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        s.push(int(3));
        s.clear();
        assert!(s.is_empty());
        assert_eq!(s.depth(), 0);
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_clear_on_empty_is_noop() {
        let mut s = CalcState::new();
        s.clear(); // must not panic or error
        assert!(s.is_empty());
    }

    // ── roll ─────────────────────────────────────────────────────────────────

    // AC-1: roll(2) on [a, b, c] — brings position 2 (b) to top → [a, c, b]
    #[test]
    fn test_roll_position2_equivalent_to_swap() {
        let mut s = CalcState::new();
        s.push(int(1)); // position 3
        s.push(int(2)); // position 2
        s.push(int(3)); // position 1 (top)
        s.roll(2).unwrap();
        assert_eq!(s.depth(), 3);
        assert_eq!(s.peek(), Some(&int(2)));   // old pos 2 is now top
        assert_eq!(s.stack[1], int(3));        // old top is now pos 2
        assert_eq!(s.stack[0], int(1));        // deep item unchanged
    }

    // AC-1: roll(3) on [a, b, c, d] — brings position 3 (b) to top → [a, c, d, b]
    #[test]
    fn test_roll_position3() {
        let mut s = CalcState::new();
        s.push(int(1)); // pos 4
        s.push(int(2)); // pos 3 — will roll to top
        s.push(int(3)); // pos 2
        s.push(int(4)); // pos 1 (top)
        s.roll(3).unwrap();
        assert_eq!(s.depth(), 4);
        assert_eq!(s.peek(), Some(&int(2)));   // rolled item at top
        assert_eq!(s.stack[2], int(4));        // old top shifted to pos 2
        assert_eq!(s.stack[1], int(3));        // shifted to pos 3
        assert_eq!(s.stack[0], int(1));        // below roll depth: unchanged
    }

    // AC-1 postconditions: depth unchanged, items below roll depth untouched
    #[test]
    fn test_roll_depth_unchanged() {
        let mut s = CalcState::new();
        for i in 1..=5 { s.push(int(i)); }
        s.roll(3).unwrap();
        assert_eq!(s.depth(), 5);
    }

    // AC-7: underflow on empty stack
    #[test]
    fn test_roll_underflow_empty() {
        let mut s = CalcState::new();
        assert!(matches!(s.roll(2), Err(CalcError::StackUnderflow)));
    }

    // AC-7: underflow with only 1 item
    #[test]
    fn test_roll_underflow_one_item() {
        let mut s = CalcState::new();
        s.push(int(42));
        assert!(matches!(s.roll(2), Err(CalcError::StackUnderflow)));
        assert_eq!(s.depth(), 1); // unchanged
    }

    // roll(n > depth) is also an underflow
    #[test]
    fn test_roll_position_beyond_depth() {
        let mut s = CalcState::new();
        s.push(int(1));
        s.push(int(2));
        assert!(matches!(s.roll(5), Err(CalcError::StackUnderflow)));
    }
}
