use crate::engine::base::Base;
use crate::engine::notation::Notation;
use dashu::float::FBig;
use dashu::integer::IBig;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CalcValue {
    Integer(IBig),
    Float(FBig),
}

impl CalcValue {
    pub fn to_f64(&self) -> f64 {
        match self {
            CalcValue::Integer(n) => n.to_string().parse::<f64>().unwrap_or(f64::NAN),
            CalcValue::Float(f) => f.to_f64().value(),
        }
    }

    pub fn from_f64(val: f64) -> CalcValue {
        // NaN and infinity cannot be represented by FBig; they become ZERO here.
        // Callers producing NaN/infinity from operations should return CalcError instead.
        CalcValue::Float(FBig::try_from(val).unwrap_or(FBig::ZERO))
    }

    #[allow(dead_code)]
    pub fn is_integer(&self) -> bool {
        matches!(self, CalcValue::Integer(_))
    }

    #[allow(dead_code)]
    pub fn to_ibig(&self) -> Option<IBig> {
        match self {
            CalcValue::Integer(n) => Some(n.clone()),
            CalcValue::Float(_) => None,
        }
    }

    /// Display with configurable float precision. Integers ignore precision.
    #[allow(dead_code)]
    pub fn display_with_precision(&self, base: Base, precision: usize) -> String {
        match self {
            CalcValue::Integer(_) => self.display_with_base(base),
            CalcValue::Float(f) => format_fbig_prec(f, precision),
        }
    }

    /// Display with notation mode (Fixed/Sci/Auto).
    /// Integers follow Sci/Auto rules using natural exact representation; precision is not applied.
    pub fn display_with_notation(&self, base: Base, precision: usize, notation: Notation) -> String {
        match self {
            CalcValue::Integer(n) => {
                let use_sci = match notation {
                    Notation::Fixed => false,
                    Notation::Sci => true,
                    Notation::Auto => {
                        let abs = n.to_string().parse::<f64>().unwrap_or(0.0).abs();
                        abs >= 1e10
                    }
                };
                if use_sci {
                    let val = n.to_string().parse::<f64>().unwrap_or(0.0);
                    format_integer_sci(val)
                } else {
                    self.display_with_base(base)
                }
            }
            CalcValue::Float(f) => format_fbig_notation(f, precision, notation),
        }
    }

    pub fn display_with_base(&self, base: Base) -> String {
        match self {
            CalcValue::Integer(n) => {
                if *n == IBig::ZERO {
                    return "0".to_string();
                }
                let negative = *n < IBig::ZERO;
                let abs_n = if negative { -n.clone() } else { n.clone() };
                let s = format_ibig_base(&abs_n, base);
                match base {
                    Base::Dec => {
                        if negative {
                            format!("-{}", s)
                        } else {
                            s
                        }
                    }
                    Base::Hex => {
                        if negative {
                            format!("-0x{}", s)
                        } else {
                            format!("0x{}", s)
                        }
                    }
                    Base::Oct => {
                        if negative {
                            format!("-0o{}", s)
                        } else {
                            format!("0o{}", s)
                        }
                    }
                    Base::Bin => {
                        if negative {
                            format!("-0b{}", s)
                        } else {
                            format!("0b{}", s)
                        }
                    }
                }
            }
            CalcValue::Float(f) => format_fbig(f),
        }
    }
}

fn format_ibig_base(n: &IBig, base: Base) -> String {
    if *n == IBig::ZERO {
        return "0".to_string();
    }
    let radix = base.radix();
    let mut digits = Vec::new();
    let mut remaining = n.clone();
    let radix_big = IBig::from(radix);
    while remaining > IBig::ZERO {
        let digit = (&remaining % &radix_big)
            .to_string()
            .parse::<u32>()
            .unwrap_or(0);
        let ch = std::char::from_digit(digit, radix).unwrap_or('?');
        digits.push(ch);
        remaining /= &radix_big;
    }
    digits.reverse();
    match base {
        Base::Hex => digits
            .iter()
            .map(|c| c.to_uppercase().next().unwrap_or(*c))
            .collect(),
        _ => digits.iter().collect(),
    }
}

pub(crate) fn format_fbig(f: &FBig) -> String {
    let val = f.to_f64().value();
    if val.is_nan() || val.is_infinite() {
        return format!("{}", val);
    }
    // Use Ryu shortest representation — avoids spurious digits from {:.15} formatting.
    format!("{}", val)
}

pub(crate) fn format_fbig_prec(f: &FBig, precision: usize) -> String {
    let val = f.to_f64().value();
    if val.is_nan() || val.is_infinite() {
        return format!("{}", val);
    }
    // Use Ryu shortest representation as the base. Only fall back to fixed decimal
    // places when the natural representation exceeds `precision` decimal places.
    // This avoids spurious digits like "122.299999999999997" caused by f64 round-trip.
    let natural = format!("{}", val);
    let natural_decimal_places = natural
        .find('.')
        .map(|pos| natural.len() - pos - 1)
        .unwrap_or(0);
    if natural_decimal_places <= precision {
        natural
    } else {
        let s = format!("{:.prec$}", val, prec = precision);
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    }
}

/// Format an integer value in scientific notation using natural exact representation.
/// e.g. 100 → "1e2", 1234 → "1.234e3", 2 → "2e0". Precision is not applied.
pub(crate) fn format_integer_sci(val: f64) -> String {
    let s = format!("{:e}", val);
    if let Some(e_pos) = s.find('e') {
        let mantissa = &s[..e_pos];
        let exponent = &s[e_pos + 1..];
        let trimmed = if mantissa.contains('.') {
            mantissa.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            mantissa.to_string()
        };
        format!("{}e{}", trimmed, exponent.trim_start_matches('+'))
    } else {
        s
    }
}

pub(crate) fn format_fbig_notation(f: &FBig, precision: usize, notation: Notation) -> String {
    let val = f.to_f64().value();
    if val.is_nan() || val.is_infinite() {
        return format!("{}", val);
    }
    let use_sci = match notation {
        Notation::Fixed => false,
        Notation::Sci => true,
        Notation::Auto => {
            let abs = val.abs();
            abs != 0.0 && (abs >= 1e10 || abs < 1e-4)
        }
    };
    if use_sci {
        // Format as scientific notation with up to `precision` digits after decimal.
        // Use Ryu natural form as base; fall back to fixed-width only when it is longer.
        let natural_e = format!("{:e}", val);
        let natural_decimal_places = natural_e
            .find('e')
            .and_then(|e| natural_e[..e].find('.').map(|d| e - d - 1))
            .unwrap_or(0);
        let s = if natural_decimal_places <= precision {
            natural_e
        } else {
            format!("{:.prec$e}", val, prec = precision)
        };
        // Normalise exponent: trim mantissa zeros and remove leading '+' from exponent.
        if let Some(e_pos) = s.find('e') {
            let mantissa = &s[..e_pos];
            let exponent = &s[e_pos + 1..];
            let trimmed = if mantissa.contains('.') {
                mantissa.trim_end_matches('0').trim_end_matches('.').to_string()
            } else {
                mantissa.to_string()
            };
            format!("{}e{}", trimmed, exponent.trim_start_matches('+'))
        } else {
            s
        }
    } else {
        format_fbig_prec(f, precision)
    }
}

impl fmt::Display for CalcValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_with_base(Base::Dec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_value_integer_serde_roundtrip() {
        let original = CalcValue::Integer(IBig::from(12345));
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: CalcValue = serde_json::from_str(&json).expect("deserialize");
        assert!(matches!(restored, CalcValue::Integer(_)));
        assert_eq!(original.to_f64(), restored.to_f64());
    }

    #[test]
    fn test_calc_value_float_serde_roundtrip() {
        let original = CalcValue::from_f64(3.14159);
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: CalcValue = serde_json::from_str(&json).expect("deserialize");
        assert!(matches!(restored, CalcValue::Float(_)));
        let diff = (original.to_f64() - restored.to_f64()).abs();
        assert!(diff < 1e-10, "float roundtrip precision lost: {}", diff);
    }

    #[test]
    fn test_calc_value_negative_integer_serde() {
        let original = CalcValue::Integer(IBig::from(-999));
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: CalcValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original.to_f64(), restored.to_f64());
    }

    // ── Story 4.4: precision-aware display ──────────────────────────────────

    #[test]
    fn test_display_with_precision_float_10() {
        // 3.141592653589793 at precision 10 → "3.1415926536" (rounded, trailing zeros trimmed)
        let val = CalcValue::from_f64(std::f64::consts::PI);
        let s = val.display_with_precision(Base::Dec, 10);
        assert_eq!(s, "3.1415926536", "precision 10 PI: got {}", s);
    }

    #[test]
    fn test_display_with_precision_trims_zeros() {
        let val = CalcValue::from_f64(3.0);
        let s = val.display_with_precision(Base::Dec, 5);
        assert_eq!(s, "3", "3.0 at precision 5 should trim to '3', got {}", s);
    }

    #[test]
    fn test_display_with_precision_integer_ignores_precision() {
        let val = CalcValue::Integer(IBig::from(42));
        let s5 = val.display_with_precision(Base::Dec, 5);
        let s15 = val.display_with_precision(Base::Dec, 15);
        assert_eq!(s5, "42");
        assert_eq!(s15, "42");
    }

    // ── AC-3: integer sci notation ───────────────────────────────────────────

    #[test]
    fn test_integer_sci_100() {
        let val = CalcValue::Integer(IBig::from(100));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Sci), "1e2");
    }

    #[test]
    fn test_integer_sci_2() {
        let val = CalcValue::Integer(IBig::from(2));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Sci), "2e0");
    }

    #[test]
    fn test_integer_sci_1234() {
        let val = CalcValue::Integer(IBig::from(1234));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Sci), "1.234e3");
    }

    #[test]
    fn test_integer_fixed_unaffected() {
        let val = CalcValue::Integer(IBig::from(100));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Fixed), "100");
    }

    // ── AC-16: auto threshold for integers ──────────────────────────────────

    #[test]
    fn test_integer_auto_above_threshold() {
        let val = CalcValue::Integer(IBig::from(10_000_000_000i64));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Auto), "1e10");
    }

    #[test]
    fn test_integer_auto_below_threshold() {
        let val = CalcValue::Integer(IBig::from(100));
        assert_eq!(val.display_with_notation(Base::Dec, 15, Notation::Auto), "100");
    }

    #[test]
    fn test_format_fbig_trims_zeros() {
        let f = FBig::try_from(3.0).unwrap();
        assert_eq!(format_fbig(&f), "3");
    }

    #[test]
    fn test_format_fbig_keeps_decimals() {
        let f = FBig::try_from(3.14).unwrap();
        let s = format_fbig(&f);
        assert!(s.starts_with("3.14"), "expected 3.14..., got {}", s);
    }
}
