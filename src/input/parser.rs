use crate::engine::units::{atoms_to_dim, atoms_to_display, lookup_alias, lookup_unit, parse_unit_expr_atoms, TaggedValue};
use crate::engine::value::CalcValue;
use crate::engine::CalcError;
use dashu::float::round::mode::Zero;
use dashu::float::Context;
use dashu::float::FBig;
use dashu::integer::IBig;

/// All known unit abbreviations sorted longest-first so longer abbrevs match before shorter ones
/// (e.g. "degF" before "F", "°F" before "F").
static UNIT_ABBREVS_BY_LENGTH: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();

fn unit_abbrevs_sorted() -> &'static Vec<&'static str> {
    UNIT_ABBREVS_BY_LENGTH.get_or_init(|| {
        // These are the canonical + alias abbreviations in descending length order
        let mut abbrevs = vec![
            "degF", "degC", "°F", "°C", "km", "mm", "cm", "ft", "yd", "mi",
            "oz", "lb", "kg", "in", "F", "C", "g", "m",
        ];
        abbrevs.sort_by_key(|a| std::cmp::Reverse(a.len()));
        abbrevs
    })
}

/// Try to parse a unit-tagged value from input like "1.9 oz", "1.9oz", "98.6F", "-1.9 kg",
/// "9.8 m/s2", "100 km/h", "80kg".
/// Returns Ok(Some(CalcValue::Tagged(...))) on success, Ok(None) if no unit found.
fn try_parse_tagged(input: &str) -> Result<Option<CalcValue>, CalcError> {
    let trimmed = input.trim();

    // 1. Try simple unit suffix matching (existing logic).
    for abbrev in unit_abbrevs_sorted() {
        if let Some(num_part) = trimmed.strip_suffix(*abbrev) {
            let num_str = num_part.trim_end();
            if num_str.is_empty() {
                continue;
            }
            if let Ok(base_val) = parse_number_only(num_str) {
                if lookup_unit(abbrev).is_none() {
                    continue;
                }
                let tagged = TaggedValue::new(base_val.to_f64(), *abbrev);
                return Ok(Some(CalcValue::Tagged(tagged)));
            }
        }
    }

    // 1.5. Try unit alias lookup (e.g. "N" → "kg*m/s2", "kph" → "km/h").
    if let Some((num_str, unit_expr)) = split_number_unit(trimmed) {
        if let Some(canonical) = lookup_alias(unit_expr) {
            if let Ok(base_val) = parse_decimal_exact(num_str) {
                if let Ok(atoms) = parse_unit_expr_atoms(canonical) {
                    if !atoms.is_empty() {
                        let dim = atoms_to_dim(&atoms);
                        let display = atoms_to_display(&atoms);
                        let tv = TaggedValue::new_compound(base_val, display, dim);
                        return Ok(Some(CalcValue::Tagged(tv)));
                    }
                }
            }
        }
    }

    // 2. Try compound unit parsing: split input into number + unit expression.
    if let Some((num_str, unit_expr)) = split_number_unit(trimmed) {
        // Only attempt compound parsing if the unit expression contains /,  *,
        // or digits after letters (e.g. "m2").
        let looks_compound = unit_expr.contains('/')
            || unit_expr.contains('*')
            || unit_expr.chars().any(|c| c.is_ascii_digit());
        if looks_compound {
            match parse_unit_expr_atoms(unit_expr) {
                Ok(atoms) if !atoms.is_empty() => {
                    if let Ok(base_val) = parse_decimal_exact(num_str) {
                        let dim = atoms_to_dim(&atoms);
                        let display = atoms_to_display(&atoms);
                        let tv = TaggedValue::new_compound(base_val, display, dim);
                        return Ok(Some(CalcValue::Tagged(tv)));
                    }
                }
                Err(e) => return Err(e), // propagate: unknown unit, malformed expression, etc.
                _ => {} // Ok(atoms) is empty — fall through
            }
        }
    }

    Ok(None)
}

/// Split input into a leading number string and a trailing unit expression.
/// Returns `Some((num_str, unit_str))` if the split is unambiguous, otherwise `None`.
fn split_number_unit(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    // Optional leading minus
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }
    // Must have at least one digit
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return None;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Optional decimal part
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    // Optional scientific notation
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let saved = i;
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        if i < bytes.len() && bytes[i].is_ascii_digit() {
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        } else {
            i = saved; // not scientific notation — backtrack
        }
    }
    let num_end = i;
    // Skip optional whitespace between number and unit
    while i < bytes.len() && bytes[i] == b' ' {
        i += 1;
    }
    let unit_start = i;
    if unit_start >= bytes.len() {
        return None; // no unit part
    }
    // Unit part must start with a letter or '°'
    let first_unit_char = s[unit_start..].chars().next()?;
    if !first_unit_char.is_alphabetic() && first_unit_char != '°' {
        return None;
    }
    Some((&s[..num_end], &s[unit_start..]))
}

/// Parse a pure number (no unit) from a string. Returns CalcValue::Integer or Float.
fn parse_number_only(s: &str) -> Result<CalcValue, CalcError> {
    let clean: String = s.chars().filter(|&c| c != '_').collect();
    if clean.is_empty() {
        return Err(CalcError::InvalidInput("Empty input".to_string()));
    }
    if let Some(rest) = clean.strip_prefix("0x").or_else(|| clean.strip_prefix("0X")) {
        return parse_hex(rest);
    }
    if let Some(rest) = clean.strip_prefix("0o").or_else(|| clean.strip_prefix("0O")) {
        return parse_octal(rest);
    }
    if let Some(rest) = clean.strip_prefix("0b").or_else(|| clean.strip_prefix("0B")) {
        return parse_binary(rest);
    }
    if let Some(rest) = clean.strip_prefix("-0x").or_else(|| clean.strip_prefix("-0X")) {
        return parse_hex(rest).map(negate);
    }
    if let Some(rest) = clean.strip_prefix("-0o").or_else(|| clean.strip_prefix("-0O")) {
        return parse_octal(rest).map(negate);
    }
    if let Some(rest) = clean.strip_prefix("-0b").or_else(|| clean.strip_prefix("-0B")) {
        return parse_binary(rest).map(negate);
    }
    let lower = clean.to_lowercase();
    if lower.contains('.') || lower.contains('e') {
        parse_float(&clean)
    } else {
        parse_integer(&clean)
    }
}

pub fn parse_value(input: &str) -> Result<CalcValue, CalcError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CalcError::InvalidInput("Empty input".to_string()));
    }
    // Skip unit detection for base-prefixed literals (hex/octal/binary).
    let lower = trimmed.to_lowercase();
    let is_prefixed = lower.starts_with("0x")
        || lower.starts_with("0o")
        || lower.starts_with("0b")
        || lower.starts_with("-0x")
        || lower.starts_with("-0o")
        || lower.starts_with("-0b");
    if !is_prefixed {
        // Try unit-tagged form first: "1.9 oz", "98.6F", "98.6 degF", etc.
        if let Some(tagged) = try_parse_tagged(trimmed)? {
            return Ok(tagged);
        }
    }
    parse_number_only(trimmed)
}

fn parse_hex(s: &str) -> Result<CalcValue, CalcError> {
    IBig::from_str_radix(s, 16)
        .map(CalcValue::Integer)
        .map_err(|_| CalcError::InvalidInput(format!("Invalid hex number: 0x{}", s)))
}

fn parse_octal(s: &str) -> Result<CalcValue, CalcError> {
    IBig::from_str_radix(s, 8)
        .map(CalcValue::Integer)
        .map_err(|_| CalcError::InvalidInput(format!("Invalid octal number: 0o{}", s)))
}

fn parse_binary(s: &str) -> Result<CalcValue, CalcError> {
    IBig::from_str_radix(s, 2)
        .map(CalcValue::Integer)
        .map_err(|_| CalcError::InvalidInput(format!("Invalid binary number: 0b{}", s)))
}

fn parse_integer(s: &str) -> Result<CalcValue, CalcError> {
    s.parse::<IBig>()
        .map(CalcValue::Integer)
        .map_err(|_| CalcError::InvalidInput(format!("Invalid integer: {}", s)))
}

/// Parse decimal precision bits — enough for ~38 significant decimal digits,
/// well beyond the 15-digit display precision. Avoids f64 intermediate.
const PARSE_PRECISION_BITS: usize = 128;

fn parse_float(s: &str) -> Result<CalcValue, CalcError> {
    parse_decimal_exact(s)
        .map(CalcValue::Float)
        .map_err(|_| CalcError::InvalidInput(format!("Invalid number: {}", s)))
}

/// Parse a decimal string (e.g. "1.223", "-4.5e-3") into FBig without
/// routing through f64, preserving full decimal precision.
fn parse_decimal_exact(s: &str) -> Result<FBig, ()> {
    // Validate the string is parseable as a number first.
    let f = s.parse::<f64>().map_err(|_| ())?;
    if f.is_nan() || f.is_infinite() {
        return Err(());
    }

    let lower = s.to_lowercase();

    // Split mantissa and exponent (e.g. "1.5e-3" → ("1.5", -3)).
    let (mantissa_s, exp_offset): (&str, i64) = match lower.find('e') {
        Some(pos) => {
            let exp: i64 = s[pos + 1..].parse().map_err(|_| ())?;
            (&s[..pos], exp)
        }
        None => (s, 0),
    };

    // Split mantissa into integer and fractional parts.
    let (int_s, frac_s) = match mantissa_s.find('.') {
        Some(pos) => (&mantissa_s[..pos], &mantissa_s[pos + 1..]),
        None => (mantissa_s, ""),
    };

    let decimal_places = frac_s.len() as i64;
    let combined = format!("{}{}", int_s, frac_s);
    let significand: IBig = combined.parse().map_err(|_| ())?;

    // net_exp: total power of 10, e.g. "1.223" → significand=1223, net_exp=-3
    let net_exp = exp_offset - decimal_places;

    let ctx = Context::<Zero>::new(PARSE_PRECISION_BITS);

    Ok(if net_exp >= 0 {
        let n = significand * IBig::from(10u8).pow(net_exp as usize);
        ctx.convert_int::<2>(n).value()
    } else {
        let num = ctx.convert_int::<2>(significand).value();
        let den = ctx.convert_int::<2>(IBig::from(10u8).pow((-net_exp) as usize)).value();
        ctx.div(num.repr(), den.repr()).value()
    })
}

fn negate(v: CalcValue) -> CalcValue {
    match v {
        CalcValue::Integer(n) => CalcValue::Integer(-n),
        CalcValue::Float(f) => CalcValue::Float(-f),
        CalcValue::Tagged(mut t) => {
            t.amount = -t.amount;
            CalcValue::Tagged(t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::CalcError;
    use dashu::integer::IBig;

    fn int_val(n: i64) -> CalcValue {
        CalcValue::Integer(IBig::from(n))
    }

    // ── integers ────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_integer_positive() {
        assert_eq!(parse_value("42"), Ok(int_val(42)));
    }

    #[test]
    fn test_parse_integer_negative() {
        assert_eq!(parse_value("-17"), Ok(int_val(-17)));
    }

    #[test]
    fn test_parse_integer_zero() {
        assert_eq!(parse_value("0"), Ok(int_val(0)));
    }

    // ── floats ──────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_float_decimal() {
        assert!(matches!(parse_value("3.14"), Ok(CalcValue::Float(_))));
    }

    #[test]
    fn test_parse_float_scientific() {
        assert!(matches!(parse_value("1.5e-3"), Ok(CalcValue::Float(_))));
    }

    #[test]
    fn test_parse_float_scientific_positive_exp() {
        assert!(matches!(parse_value("1e10"), Ok(CalcValue::Float(_))));
    }

    // ── hex ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_hex_uppercase() {
        assert_eq!(parse_value("0xFF"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_hex_lowercase() {
        assert_eq!(parse_value("0xff"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_hex_prefix_uppercase() {
        assert_eq!(parse_value("0XFF"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_hex_negative() {
        assert_eq!(parse_value("-0xFF"), Ok(int_val(-255)));
    }

    // ── octal ────────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_octal() {
        assert_eq!(parse_value("0o377"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_octal_prefix_uppercase() {
        assert_eq!(parse_value("0O377"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_octal_negative() {
        assert_eq!(parse_value("-0o10"), Ok(int_val(-8)));
    }

    // ── binary ───────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_value("0b11111111"), Ok(int_val(255)));
    }

    #[test]
    fn test_parse_binary_prefix_uppercase() {
        assert_eq!(parse_value("0B101"), Ok(int_val(5)));
    }

    #[test]
    fn test_parse_binary_negative() {
        assert_eq!(parse_value("-0b101"), Ok(int_val(-5)));
    }

    // ── digit separators ────────────────────────────────────────────────────

    #[test]
    fn test_parse_digit_separators_integer() {
        assert_eq!(parse_value("1_000_000"), Ok(int_val(1_000_000)));
    }

    #[test]
    fn test_parse_digit_separators_hex() {
        assert_eq!(parse_value("0xFF_FF"), Ok(int_val(65535)));
    }

    // ── errors ───────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_empty_string() {
        assert!(matches!(parse_value(""), Err(CalcError::InvalidInput(_))));
    }

    #[test]
    fn test_parse_garbage() {
        assert!(matches!(
            parse_value("abc"),
            Err(CalcError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_parse_invalid_hex() {
        assert!(matches!(
            parse_value("0xGG"),
            Err(CalcError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_parse_invalid_octal() {
        assert!(matches!(
            parse_value("0o99"),
            Err(CalcError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_parse_invalid_binary() {
        assert!(matches!(
            parse_value("0b2"),
            Err(CalcError::InvalidInput(_))
        ));
    }

    // ── compound unit parsing ─────────────────────────────────────────────────

    #[test]
    fn test_parse_compound_speed() {
        // AC-1: "100 km/h" parses as a tagged value with unit "km/h"
        let result = parse_value("100 km/h").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "km/h");
                assert!((tv.amount.to_f64().value() - 100.0).abs() < 1e-9);
            }
            _ => panic!("expected Tagged, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_compound_acceleration() {
        // AC-2: "9.8 m/s2" parses as tagged with unit "m/s2"
        let result = parse_value("9.8 m/s2").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "m/s2");
                assert!((tv.amount.to_f64().value() - 9.8).abs() < 1e-6);
            }
            _ => panic!("expected Tagged"),
        }
    }

    #[test]
    fn test_parse_compound_area() {
        // "25 m2" parses as tagged with unit "m2"
        let result = parse_value("25 m2").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "m2");
                assert!((tv.amount.to_f64().value() - 25.0).abs() < 1e-9);
            }
            _ => panic!("expected Tagged"),
        }
    }

    #[test]
    fn test_parse_compound_no_space() {
        // "100km/h" (no space) parses as tagged with unit "km/h"
        let result = parse_value("100km/h").unwrap();
        match result {
            CalcValue::Tagged(tv) => assert_eq!(tv.unit, "km/h"),
            _ => panic!("expected Tagged"),
        }
    }

    #[test]
    fn test_parse_unknown_unit_error() {
        // AC-12: "9.8 m/fathom2" should error with "unknown unit: fathom"
        let result = parse_value("9.8 m/fathom2");
        assert!(
            matches!(&result, Err(CalcError::InvalidInput(e)) if e.contains("unknown unit: fathom")),
            "got: {:?}", result
        );
    }

    // AC-18: malformed compound unit expression raises "invalid unit expression" error
    #[test]
    fn test_parse_malformed_compound_unit_errors() {
        let result = parse_value("9.8 m//s");
        assert!(
            matches!(&result, Err(CalcError::InvalidInput(e)) if e.contains("invalid unit expression")),
            "double-slash should raise invalid unit expression error, got: {:?}", result
        );
    }

    // AC-18: trailing slash also malformed
    #[test]
    fn test_parse_trailing_slash_errors() {
        let result = parse_value("5 m/");
        assert!(
            matches!(&result, Err(CalcError::InvalidInput(e)) if e.contains("invalid unit expression")),
            "trailing slash should raise invalid unit expression error, got: {:?}", result
        );
    }

    #[test]
    fn test_parse_compound_force() {
        // "80 kg*m/s2"
        let result = parse_value("80 kg*m/s2").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "kg*m/s2");
                assert!((tv.amount.to_f64().value() - 80.0).abs() < 1e-9);
            }
            _ => panic!("expected Tagged"),
        }
    }

    // ── unit alias parsing ────────────────────────────────────────────────────

    // AC-1: "9.8 N" resolves to TaggedValue with canonical unit "kg*m/s2"
    #[test]
    fn test_parse_alias_newton() {
        let result = parse_value("9.8 N").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "kg*m/s2", "N should resolve to kg*m/s2, got {}", tv.unit);
                assert!((tv.amount.to_f64().value() - 9.8).abs() < 1e-6);
            }
            _ => panic!("expected Tagged, got {:?}", result),
        }
    }

    // AC-4: "100 kph" resolves to TaggedValue with canonical unit "km/h"
    #[test]
    fn test_parse_alias_kph() {
        let result = parse_value("100 kph").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "km/h", "kph should resolve to km/h, got {}", tv.unit);
                assert!((tv.amount.to_f64().value() - 100.0).abs() < 1e-9);
            }
            _ => panic!("expected Tagged, got {:?}", result),
        }
    }

    // AC-6: unknown unit string still errors (alias path doesn't shadow the error)
    #[test]
    fn test_parse_unknown_still_errors() {
        // Plain unknown suffix (no / or *): falls through to number parser → InvalidInput
        let result = parse_value("9.8 xyz");
        assert!(
            matches!(&result, Err(CalcError::InvalidInput(_))),
            "unknown unit string should error, got: {:?}", result
        );
        // Unknown compound unit: explicitly "unknown unit" message
        let result2 = parse_value("9.8 xyz/s2");
        assert!(
            matches!(&result2, Err(CalcError::InvalidInput(e)) if e.contains("unknown unit")),
            "unknown compound unit should report unknown unit, got: {:?}", result2
        );
    }

    // AC-7: direct compound entry ("9.8 kg*m/s2") produces same canonical unit as alias "N"
    #[test]
    fn test_parse_direct_compound_unchanged() {
        let alias_result = parse_value("9.8 N").unwrap();
        let direct_result = parse_value("9.8 kg*m/s2").unwrap();
        match (alias_result, direct_result) {
            (CalcValue::Tagged(a), CalcValue::Tagged(d)) => {
                assert_eq!(a.unit, d.unit, "alias and direct entry should produce same unit");
                assert_eq!(a.dim, d.dim, "alias and direct entry should produce same DimensionVector");
            }
            _ => panic!("expected both to be Tagged"),
        }
    }

    // AC-9: alias-resolved value stores canonical unit string (not the alias itself)
    #[test]
    fn test_alias_stores_canonical_unit() {
        let result = parse_value("5 N").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_ne!(tv.unit, "N", "alias should not be stored as unit label");
                assert_eq!(tv.unit, "kg*m/s2", "canonical form should be stored");
            }
            _ => panic!("expected Tagged"),
        }
    }

    // Pa alias resolves correctly
    #[test]
    fn test_parse_alias_pa() {
        let result = parse_value("101325 Pa").unwrap();
        match result {
            CalcValue::Tagged(tv) => {
                assert_eq!(tv.unit, "kg/m*s2", "Pa should resolve to kg/m*s2, got {}", tv.unit);
            }
            _ => panic!("expected Tagged"),
        }
    }

    // ── precision regression ─────────────────────────────────────────────────

    #[test]
    fn test_parse_decimal_no_f64_precision_loss() {
        // 1.223 × 100 must display as "122.3", not "122.299999999999997".
        // Regression: old code parsed through f64, introducing IEEE 754 error.
        use crate::engine::ops::{apply_op, Op};
        use crate::engine::stack::CalcState;
        use crate::engine::value::format_fbig_prec;

        let mut state = CalcState::default();
        let v1 = parse_value("1.223").unwrap();
        let v2 = parse_value("100").unwrap();
        state.stack.push(v1);
        state.stack.push(v2);
        apply_op(&mut state, Op::Mul).unwrap();
        let result = state.stack.last().unwrap();
        if let crate::engine::value::CalcValue::Float(f) = result {
            let s = format_fbig_prec(f, 15);
            assert_eq!(s, "122.3", "1.223 × 100 should display as '122.3', got '{}'", s);
        } else {
            panic!("expected Float result");
        }
    }
}
