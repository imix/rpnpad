use crate::engine::{angle::AngleMode, base::Base, notation::Notation};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".rpncalc").join("config.toml"))
}

/// Intermediate deserialization struct — all fields Optional so partial configs
/// are gracefully handled; missing keys fall back to Config::default().
#[derive(Deserialize)]
struct ConfigToml {
    angle_mode: Option<String>,
    base: Option<String>,
    notation: Option<String>,
    precision: Option<usize>,
    max_undo_history: Option<usize>,
    persist_session: Option<bool>,
}

pub struct Config {
    pub angle_mode: AngleMode,
    pub base: Base,
    pub notation: Notation,
    pub precision: usize,
    pub max_undo_history: usize,
    pub persist_session: bool,
}

/// Core load — testable with injected path.
/// Returns Config::default() if file is missing, unreadable, or malformed TOML.
/// Invalid field values are silently ignored; the default is used for that field.
pub(crate) fn load_from_path(path: &Path) -> Config {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Config::default(), // file not found or unreadable
    };
    let parsed: ConfigToml = match toml::from_str(&content) {
        Ok(t) => t,
        Err(_) => return Config::default(), // malformed TOML
    };
    let mut cfg = Config::default();
    if let Some(am) = parsed.angle_mode {
        match am.to_lowercase().as_str() {
            "rad" => cfg.angle_mode = AngleMode::Rad,
            "grad" => cfg.angle_mode = AngleMode::Grad,
            "deg" => cfg.angle_mode = AngleMode::Deg,
            _ => {} // invalid value — keep default
        }
    }
    if let Some(b) = parsed.base {
        match b.to_lowercase().as_str() {
            "hex" => cfg.base = Base::Hex,
            "oct" => cfg.base = Base::Oct,
            "bin" => cfg.base = Base::Bin,
            "dec" => cfg.base = Base::Dec,
            _ => {} // invalid value — keep default
        }
    }
    if let Some(n) = parsed.notation {
        match n.to_lowercase().as_str() {
            "sci" => cfg.notation = Notation::Sci,
            "auto" => cfg.notation = Notation::Auto,
            "fixed" => cfg.notation = Notation::Fixed,
            _ => {} // invalid value — keep default
        }
    }
    if let Some(p) = parsed.precision {
        if p > 0 {
            cfg.precision = p;
        } // 0 is not a valid precision — keep default
    }
    if let Some(d) = parsed.max_undo_history {
        if d > 0 {
            cfg.max_undo_history = d;
        }
    }
    if let Some(s) = parsed.persist_session {
        cfg.persist_session = s;
    }
    cfg
}

impl Config {
    /// Load config from ~/.rpncalc/config.toml.
    /// Falls back to defaults if the file is missing or invalid.
    pub fn load() -> Self {
        let path = match config_path() {
            Some(p) => p,
            None => return Self::default(),
        };
        load_from_path(&path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            angle_mode: AngleMode::Deg,
            base: Base::Dec,
            notation: Notation::Fixed,
            precision: 15,
            max_undo_history: 1000,
            persist_session: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{angle::AngleMode, base::Base};

    fn write_temp_toml(name: &str, content: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }

    fn cleanup(path: &std::path::Path) {
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_config_defaults() {
        let cfg = Config::default();
        assert_eq!(cfg.angle_mode, AngleMode::Deg);
        assert_eq!(cfg.base, Base::Dec);
        assert_eq!(cfg.precision, 15);
        assert_eq!(cfg.max_undo_history, 1000);
        assert!(cfg.persist_session);
    }

    #[test]
    fn test_load_angle_rad() {
        let path = write_temp_toml("rpncalc_cfg_angle_rad.toml", r#"angle_mode = "rad""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Rad);
        assert_eq!(cfg.base, Base::Dec);
        assert_eq!(cfg.precision, 15);
    }

    #[test]
    fn test_load_angle_grad() {
        let path = write_temp_toml("rpncalc_cfg_angle_grad.toml", r#"angle_mode = "grad""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Grad);
    }

    #[test]
    fn test_load_angle_case_insensitive() {
        let path = write_temp_toml("rpncalc_cfg_angle_ci.toml", r#"angle_mode = "RAD""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Rad);
    }

    #[test]
    fn test_load_base_hex() {
        let path = write_temp_toml("rpncalc_cfg_base_hex.toml", r#"base = "hex""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.base, Base::Hex);
    }

    #[test]
    fn test_load_base_oct() {
        let path = write_temp_toml("rpncalc_cfg_base_oct.toml", r#"base = "oct""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.base, Base::Oct);
    }

    #[test]
    fn test_load_base_bin() {
        let path = write_temp_toml("rpncalc_cfg_base_bin.toml", r#"base = "bin""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.base, Base::Bin);
    }

    #[test]
    fn test_load_precision() {
        let path = write_temp_toml("rpncalc_cfg_prec.toml", "precision = 10");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.precision, 10);
    }

    #[test]
    fn test_load_undo_depth() {
        let path = write_temp_toml("rpncalc_cfg_undo.toml", "max_undo_history = 50");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.max_undo_history, 50);
    }

    #[test]
    fn test_load_persist_false() {
        let path = write_temp_toml("rpncalc_cfg_persist.toml", "persist_session = false");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert!(!cfg.persist_session);
    }

    #[test]
    fn test_missing_file_uses_defaults() {
        let path = std::env::temp_dir().join("rpncalc_cfg_nonexistent_4321.toml");
        let _ = std::fs::remove_file(&path); // ensure absent
        let cfg = load_from_path(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Deg);
        assert_eq!(cfg.base, Base::Dec);
        assert_eq!(cfg.precision, 15);
        assert_eq!(cfg.max_undo_history, 1000);
        assert!(cfg.persist_session);
    }

    #[test]
    fn test_invalid_angle_value_uses_default() {
        let path = write_temp_toml("rpncalc_cfg_bad_angle.toml", r#"angle_mode = "invalid""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Deg); // unchanged default
    }

    #[test]
    fn test_invalid_toml_uses_defaults() {
        let path = write_temp_toml("rpncalc_cfg_badtoml.toml", "not valid toml [[[ !!!");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.angle_mode, AngleMode::Deg);
        assert_eq!(cfg.precision, 15);
    }

    #[test]
    fn test_load_precision_zero_uses_default() {
        let path = write_temp_toml("rpncalc_cfg_prec_zero.toml", "precision = 0");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.precision, 15); // 0 is invalid — keep default
    }

    #[test]
    fn test_partial_config_keeps_other_defaults() {
        let path = write_temp_toml("rpncalc_cfg_partial.toml", "precision = 5");
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.precision, 5);
        assert_eq!(cfg.angle_mode, AngleMode::Deg);
        assert_eq!(cfg.base, Base::Dec);
        assert_eq!(cfg.max_undo_history, 1000);
        assert!(cfg.persist_session);
    }

    // ── notation config key ─────────────────────────────────────────────────

    #[test]
    fn test_load_notation_sci() {
        use crate::engine::notation::Notation;
        let path = write_temp_toml("rpncalc_cfg_notation_sci.toml", r#"notation = "sci""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.notation, Notation::Sci);
    }

    #[test]
    fn test_load_notation_auto() {
        use crate::engine::notation::Notation;
        let path = write_temp_toml("rpncalc_cfg_notation_auto.toml", r#"notation = "auto""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.notation, Notation::Auto);
    }

    #[test]
    fn test_load_notation_fixed() {
        use crate::engine::notation::Notation;
        let path = write_temp_toml("rpncalc_cfg_notation_fixed.toml", r#"notation = "fixed""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.notation, Notation::Fixed);
    }

    #[test]
    fn test_notation_default_is_fixed() {
        use crate::engine::notation::Notation;
        let cfg = Config::default();
        assert_eq!(cfg.notation, Notation::Fixed);
    }

    #[test]
    fn test_load_notation_invalid_uses_default() {
        use crate::engine::notation::Notation;
        let path = write_temp_toml("rpncalc_cfg_notation_bad.toml", r#"notation = "exponential""#);
        let cfg = load_from_path(&path);
        cleanup(&path);
        assert_eq!(cfg.notation, Notation::Fixed); // invalid → default
    }
}
