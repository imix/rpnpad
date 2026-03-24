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
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AppMode {
    Normal,
    Insert(String),
    Alpha(String),
    AlphaStore(String),
    Chord(ChordCategory),
}
