use serde::{Deserialize, Serialize};
use std::fmt;

/// Controls how floating-point values are displayed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Notation {
    /// Always use fixed-point (e.g. `3.14159`). Default.
    Fixed,
    /// Always use scientific notation (e.g. `3.14159e0`).
    Sci,
    /// Fixed below threshold, sci above. Threshold: |v| ≥ 1e10 or (|v| < 1e-4 and v ≠ 0).
    Auto,
}

impl Default for Notation {
    fn default() -> Self {
        Notation::Fixed
    }
}

impl fmt::Display for Notation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Notation::Fixed => write!(f, "FIX"),
            Notation::Sci => write!(f, "SCI"),
            Notation::Auto => write!(f, "AUTO"),
        }
    }
}
