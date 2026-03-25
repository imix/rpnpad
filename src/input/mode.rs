#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ChordCategory {
    Trig,
    Log,
    Functions,
    Constants,
    AngleMode,
    Base,
    HexStyle,
    Rounding,
    Config,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AppMode {
    Normal,
    Insert(String),
    Alpha(String),
    AlphaStore(String),
    Chord(ChordCategory),
    /// Cursor-based stack navigation. The `usize` is the 1-indexed cursor
    /// position from the top (always ≥ 2; position 1 is the top itself).
    Browse(usize),
    /// Precision entry sub-mode. Buffer accumulates up to 2 digit chars.
    PrecisionInput(String),
}
