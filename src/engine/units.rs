use crate::engine::error::CalcError;
use dashu::float::{round::mode::Zero, Context, FBig};
use dashu::integer::IBig;
use serde::{Deserialize, Serialize};

/// Dimension vector: signed integer exponents for the seven SI base dimensions.
/// All-zeros represents a dimensionless value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DimensionVector {
    #[serde(default, skip_serializing_if = "is_zero")]
    pub kg: i8, // mass
    #[serde(default, skip_serializing_if = "is_zero")]
    pub m: i8, // length
    #[serde(default, skip_serializing_if = "is_zero")]
    pub s: i8, // time
    #[serde(default, skip_serializing_if = "is_zero", rename = "A")]
    pub a: i8, // electric current
    #[serde(default, skip_serializing_if = "is_zero", rename = "K")]
    pub k: i8, // thermodynamic temperature
    #[serde(default, skip_serializing_if = "is_zero")]
    pub mol: i8, // amount of substance
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cd: i8, // luminous intensity
}

fn is_zero(n: &i8) -> bool {
    *n == 0
}

impl DimensionVector {
    pub fn is_dimensionless(&self) -> bool {
        *self == Self::default()
    }

    /// Add dimension exponents — used for multiplication.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            kg: self.kg + other.kg,
            m: self.m + other.m,
            s: self.s + other.s,
            a: self.a + other.a,
            k: self.k + other.k,
            mol: self.mol + other.mol,
            cd: self.cd + other.cd,
        }
    }

    /// Subtract dimension exponents — used for division.
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            kg: self.kg - other.kg,
            m: self.m - other.m,
            s: self.s - other.s,
            a: self.a - other.a,
            k: self.k - other.k,
            mol: self.mol - other.mol,
            cd: self.cd - other.cd,
        }
    }

    /// Negate all exponents — used for reciprocal (1/x).
    pub fn negate(&self) -> Self {
        Self {
            kg: -self.kg,
            m: -self.m,
            s: -self.s,
            a: -self.a,
            k: -self.k,
            mol: -self.mol,
            cd: -self.cd,
        }
    }

    /// Halve all exponents — used for sqrt. Returns `None` if any exponent is odd.
    pub fn halve(&self) -> Option<Self> {
        if self.kg % 2 != 0
            || self.m % 2 != 0
            || self.s % 2 != 0
            || self.a % 2 != 0
            || self.k % 2 != 0
            || self.mol % 2 != 0
            || self.cd % 2 != 0
        {
            return None;
        }
        Some(Self {
            kg: self.kg / 2,
            m: self.m / 2,
            s: self.s / 2,
            a: self.a / 2,
            k: self.k / 2,
            mol: self.mol / 2,
            cd: self.cd / 2,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitCategory {
    Weight,
    Length,
    Temperature,
    Time,
}

impl UnitCategory {
    pub fn name(&self) -> &'static str {
        match self {
            UnitCategory::Weight => "weight",
            UnitCategory::Length => "length",
            UnitCategory::Temperature => "temperature",
            UnitCategory::Time => "time",
        }
    }
}

/// A physical unit. Linear units have a `to_base` scale factor as an exact
/// decimal string; temperature uses None and is handled by affine conversion.
pub struct Unit {
    pub abbrev: &'static str,
    /// Display abbreviation (may differ from abbrev for aliases).
    pub display: &'static str,
    pub category: UnitCategory,
    /// Scale factor to base unit as exact decimal string. None for temperature (affine).
    pub to_base: Option<&'static str>,
    /// SI dimension vector for this unit.
    pub dim: DimensionVector,
}

/// All recognised units. Aliases (e.g. "F" for "°F") have the same
/// display as their canonical form but a different abbrev.
static UNITS: &[Unit] = &[
    // ── Weight (base: kg) ────────────────────────────────────────────────────
    Unit { abbrev: "oz",  display: "oz",  category: UnitCategory::Weight,      to_base: Some("0.028349523125"), dim: DimensionVector { kg: 1, m: 0, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "lb",  display: "lb",  category: UnitCategory::Weight,      to_base: Some("0.45359237"),     dim: DimensionVector { kg: 1, m: 0, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "g",   display: "g",   category: UnitCategory::Weight,      to_base: Some("0.001"),          dim: DimensionVector { kg: 1, m: 0, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "kg",  display: "kg",  category: UnitCategory::Weight,      to_base: Some("1"),              dim: DimensionVector { kg: 1, m: 0, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    // ── Length (base: m) ─────────────────────────────────────────────────────
    Unit { abbrev: "mm",  display: "mm",  category: UnitCategory::Length,      to_base: Some("0.001"),          dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "cm",  display: "cm",  category: UnitCategory::Length,      to_base: Some("0.01"),           dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "m",   display: "m",   category: UnitCategory::Length,      to_base: Some("1"),              dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "km",  display: "km",  category: UnitCategory::Length,      to_base: Some("1000"),           dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "in",  display: "in",  category: UnitCategory::Length,      to_base: Some("0.0254"),         dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "ft",  display: "ft",  category: UnitCategory::Length,      to_base: Some("0.3048"),         dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "yd",  display: "yd",  category: UnitCategory::Length,      to_base: Some("0.9144"),         dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "mi",  display: "mi",  category: UnitCategory::Length,      to_base: Some("1609.344"),       dim: DimensionVector { kg: 0, m: 1, s: 0, a: 0, k: 0, mol: 0, cd: 0 } },
    // ── Time (base: s) ───────────────────────────────────────────────────────
    Unit { abbrev: "s",   display: "s",   category: UnitCategory::Time,        to_base: Some("1"),              dim: DimensionVector { kg: 0, m: 0, s: 1, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "min", display: "min", category: UnitCategory::Time,        to_base: Some("60"),             dim: DimensionVector { kg: 0, m: 0, s: 1, a: 0, k: 0, mol: 0, cd: 0 } },
    Unit { abbrev: "h",   display: "h",   category: UnitCategory::Time,        to_base: Some("3600"),           dim: DimensionVector { kg: 0, m: 0, s: 1, a: 0, k: 0, mol: 0, cd: 0 } },
    // ── Temperature (affine) ─────────────────────────────────────────────────
    Unit { abbrev: "°F",  display: "°F",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
    Unit { abbrev: "°C",  display: "°C",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
    // ASCII aliases — same display as canonical but typable without special chars
    Unit { abbrev: "F",   display: "°F",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
    Unit { abbrev: "C",   display: "°C",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
    Unit { abbrev: "degF",display: "°F",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
    Unit { abbrev: "degC",display: "°C",  category: UnitCategory::Temperature, to_base: None,                   dim: DimensionVector { kg: 0, m: 0, s: 0, a: 0, k: 1, mol: 0, cd: 0 } },
];

/// Parse an exact decimal string (e.g. "0.3048") to FBig at 128-bit precision,
/// without routing through f64. Used for unit scale factors.
fn parse_scale(s: &str) -> FBig {
    let (int_s, frac_s) = match s.find('.') {
        Some(pos) => (&s[..pos], &s[pos + 1..]),
        None => (s, ""),
    };
    let decimal_places = frac_s.len() as i64;
    let combined = format!("{}{}", int_s, frac_s);
    let significand: IBig = combined.parse().expect("valid scale constant");
    let ctx = Context::<Zero>::new(128);
    if decimal_places == 0 {
        ctx.convert_int::<2>(significand).value()
    } else {
        let num = ctx.convert_int::<2>(significand).value();
        let den = ctx.convert_int::<2>(IBig::from(10u8).pow(decimal_places as usize)).value();
        ctx.div(num.repr(), den.repr()).value()
    }
}

/// Convenience: parse an integer constant (32, 5, 9 …) to FBig.
fn fbig_int(n: i64) -> FBig {
    let ctx = Context::<Zero>::new(128);
    ctx.convert_int::<2>(IBig::from(n)).value()
}

/// Look up a unit by abbreviation (case-sensitive). Returns the first match.
pub fn lookup_unit(abbrev: &str) -> Option<&'static Unit> {
    UNITS.iter().find(|u| u.abbrev == abbrev)
}

// ── Unit aliases ──────────────────────────────────────────────────────────────

/// Common unit aliases: short names that map to canonical compound expressions.
/// Checked before the compound parser so e.g. "N" is resolved without needing
/// the user to type "kg*m/s2".
pub static UNIT_ALIASES: &[(&str, &str)] = &[
    ("N",   "kg*m/s2"),   // newton  — force
    ("kph", "km/h"),      // speed   — kilometres per hour
    ("Pa",  "kg/m*s2"),   // pascal  — pressure (kg·m⁻¹·s⁻²)
    ("J",   "kg*m2/s2"),  // joule   — energy
    ("W",   "kg*m2/s3"),  // watt    — power
];

/// Look up an alias by its short name. Returns the canonical compound string.
pub fn lookup_alias(s: &str) -> Option<&'static str> {
    UNIT_ALIASES.iter().find(|(alias, _)| *alias == s).map(|(_, canonical)| *canonical)
}

/// Return the alias names whose resolved DimensionVector equals `dim`.
/// Used by the hints pane to surface named conversion targets.
pub fn aliases_for_dim(dim: &DimensionVector) -> Vec<&'static str> {
    UNIT_ALIASES
        .iter()
        .filter(|(_, canonical)| {
            parse_unit_expr_atoms(canonical)
                .ok()
                .map(|atoms| atoms_to_dim(&atoms) == *dim)
                .unwrap_or(false)
        })
        .map(|(alias, _)| *alias)
        .collect()
}

// ── Compound unit helpers ─────────────────────────────────────────────────────

/// Parse a single unit atom: `<abbrev>[<exponent>]`.
/// Abbrev is the leading alphabetic (and `°`) characters.
/// Exponent is trailing signed integer digits (default 1).
/// Temperature units are rejected in compound expressions.
fn parse_unit_atom(s: &str) -> Result<(String, i8), CalcError> {
    if s.is_empty() {
        return Err(CalcError::InvalidInput("empty unit atom".to_string()));
    }
    let chars: Vec<char> = s.chars().collect();
    // Abbrev: leading letters and '°'
    let mut abbrev_end = 0;
    while abbrev_end < chars.len() && (chars[abbrev_end].is_alphabetic() || chars[abbrev_end] == '°') {
        abbrev_end += 1;
    }
    if abbrev_end == 0 {
        return Err(CalcError::InvalidInput(format!("invalid unit atom: {}", s)));
    }
    let abbrev: String = chars[..abbrev_end].iter().collect();
    let exp_str: String = chars[abbrev_end..].iter().collect();
    let exp: i8 = if exp_str.is_empty() {
        1
    } else {
        exp_str.parse::<i8>().map_err(|_| {
            CalcError::InvalidInput(format!("invalid exponent in unit atom: {}", s))
        })?
    };
    let unit = lookup_unit(&abbrev).ok_or_else(|| {
        CalcError::InvalidInput(format!("unknown unit: {}", abbrev))
    })?;
    if unit.category == UnitCategory::Temperature {
        return Err(CalcError::InvalidInput(format!(
            "temperature unit {} cannot be used in compound expressions",
            abbrev
        )));
    }
    Ok((abbrev, exp))
}

/// Split an atom string by `*` or whitespace.
fn split_atom_tokens(s: &str) -> Vec<String> {
    s.replace('*', " ")
        .split_whitespace()
        .map(|a| a.to_string())
        .collect()
}

/// Merge atom into list, adding exponents for matching abbrevs.
fn add_atom_to_list(atoms: &mut Vec<(String, i8)>, abbrev: String, exp: i8) {
    if let Some(existing) = atoms.iter_mut().find(|(a, _)| *a == abbrev) {
        existing.1 += exp;
    } else {
        atoms.push((abbrev, exp));
    }
}

/// Parse a compound unit expression into a list of (abbrev, signed-exponent) pairs.
/// Grammar: `<numerator> [ "/" <denominator> ]`
/// Each part is split on `*` / whitespace for atoms: `<abbrev>[<exponent>]`.
pub fn parse_unit_expr_atoms(expr: &str) -> Result<Vec<(String, i8)>, CalcError> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Err(CalcError::InvalidInput("empty unit expression".to_string()));
    }
    let slash_pos = expr.find('/');
    let (num_str, den_str) = match slash_pos {
        None => (expr, None),
        Some(pos) => {
            let den = &expr[pos + 1..];
            if den.is_empty() {
                return Err(CalcError::InvalidInput(format!("invalid unit expression: {}", expr)));
            }
            // Reject double-slash
            if den.contains('/') {
                return Err(CalcError::InvalidInput(format!("invalid unit expression: {}", expr)));
            }
            (&expr[..pos], Some(den))
        }
    };

    let mut atoms: Vec<(String, i8)> = Vec::new();

    for token in split_atom_tokens(num_str) {
        let (abbrev, exp) = parse_unit_atom(&token)?;
        if exp != 0 {
            add_atom_to_list(&mut atoms, abbrev, exp);
        }
    }

    if let Some(den) = den_str {
        for token in split_atom_tokens(den) {
            let (abbrev, exp) = parse_unit_atom(&token)?;
            if exp != 0 {
                add_atom_to_list(&mut atoms, abbrev, -exp);
            }
        }
    }

    // Drop any that cancelled to zero
    atoms.retain(|(_, e)| *e != 0);
    Ok(atoms)
}

/// Compute the DimensionVector from a list of (abbrev, exponent) atoms.
pub fn atoms_to_dim(atoms: &[(String, i8)]) -> DimensionVector {
    let mut dim = DimensionVector::default();
    for (abbrev, exp) in atoms {
        let exp = *exp;
        if let Some(unit) = lookup_unit(abbrev) {
            dim.kg += unit.dim.kg * exp;
            dim.m += unit.dim.m * exp;
            dim.s += unit.dim.s * exp;
            dim.a += unit.dim.a * exp;
            dim.k += unit.dim.k * exp;
            dim.mol += unit.dim.mol * exp;
            dim.cd += unit.dim.cd * exp;
        }
    }
    dim
}

/// Format a list of atoms as a display string.
/// Positive exponents → numerator joined by `*`.
/// Negative exponents → denominator joined by `*` with absolute exponents.
pub fn atoms_to_display(atoms: &[(String, i8)]) -> String {
    let mut num_parts: Vec<String> = Vec::new();
    let mut den_parts: Vec<String> = Vec::new();

    for (abbrev, exp) in atoms.iter().filter(|(_, e)| *e != 0) {
        let exp = *exp;
        if exp > 0 {
            if exp == 1 {
                num_parts.push(abbrev.clone());
            } else {
                num_parts.push(format!("{}{}", abbrev, exp));
            }
        } else {
            let abs_exp = (-exp) as u8;
            if abs_exp == 1 {
                den_parts.push(abbrev.clone());
            } else {
                den_parts.push(format!("{}{}", abbrev, abs_exp));
            }
        }
    }

    match (num_parts.is_empty(), den_parts.is_empty()) {
        (true, true) => String::new(),
        (false, true) => num_parts.join("*"),
        (true, false) => format!("1/{}", den_parts.join("*")),
        (false, false) => format!("{}/{}", num_parts.join("*"), den_parts.join("*")),
    }
}

/// Combine two atom lists for multiplication (add exponents, drop zeros).
pub fn combine_atoms_mul(a: &[(String, i8)], b: &[(String, i8)]) -> Vec<(String, i8)> {
    let mut result: Vec<(String, i8)> = a.to_vec();
    for (abbrev, exp) in b {
        add_atom_to_list(&mut result, abbrev.clone(), *exp);
    }
    result.retain(|(_, e)| *e != 0);
    result
}

/// Raise a scale factor FBig to an i8 exponent (small integers only, practical range ±8).
fn fbig_pow_i8(base: FBig, exp: i8) -> FBig {
    if exp == 0 {
        return fbig_int(1);
    }
    let abs = exp.unsigned_abs() as usize;
    let mut result = base.clone();
    for _ in 1..abs {
        result = result * base.clone();
    }
    if exp < 0 {
        fbig_int(1) / result
    } else {
        result
    }
}

/// Compute the SI scale factor for a compound unit expression.
/// E.g., atoms `[("km", 1), ("h", -1)]` → `1000 / 3600`.
pub fn compound_to_si_scale(atoms: &[(String, i8)]) -> Result<FBig, CalcError> {
    let mut scale = fbig_int(1);
    for (abbrev, exp) in atoms {
        let exp = *exp;
        let unit = lookup_unit(abbrev).ok_or_else(|| {
            CalcError::InvalidInput(format!("unknown unit: {}", abbrev))
        })?;
        let to_base_str = unit.to_base.ok_or_else(|| {
            CalcError::IncompatibleUnits(format!("{} has no linear conversion", abbrev))
        })?;
        let unit_scale = parse_scale(to_base_str);
        scale = scale * fbig_pow_i8(unit_scale, exp);
    }
    Ok(scale)
}


/// Fallback unit display derived from a DimensionVector, using SI base abbreviations.
pub fn derive_display_from_dim(dim: &DimensionVector) -> String {
    let mut atoms: Vec<(String, i8)> = Vec::new();
    if dim.kg != 0 { atoms.push(("kg".to_string(), dim.kg)); }
    if dim.m != 0 { atoms.push(("m".to_string(), dim.m)); }
    if dim.s != 0 { atoms.push(("s".to_string(), dim.s)); }
    if dim.mol != 0 { atoms.push(("mol".to_string(), dim.mol)); }
    atoms_to_display(&atoms)
}

/// Convert `from`'s amount into `to`'s unit scale.
/// Handles both simple (category-based) and compound units.
pub fn convert_tagged_to_unit(from: &TaggedValue, to: &TaggedValue) -> Result<FBig, CalcError> {
    if from.unit == to.unit {
        return Ok(from.amount.clone());
    }
    if from.dim != to.dim {
        return Err(CalcError::IncompatibleUnits(format!(
            "incompatible units: {} and {}", from.unit, to.unit
        )));
    }
    // Try simple unit conversion
    match (lookup_unit(&from.unit), lookup_unit(&to.unit)) {
        (Some(from_unit), Some(to_unit)) => {
            convert(from.amount.clone(), from_unit, to_unit)
        }
        _ => {
            // Compound conversion via SI scale factors
            let from_atoms = parse_unit_expr_atoms(&from.unit)
                .map_err(|_| CalcError::IncompatibleUnits(format!("unknown unit: {}", from.unit)))?;
            let to_atoms = parse_unit_expr_atoms(&to.unit)
                .map_err(|_| CalcError::IncompatibleUnits(format!("unknown unit: {}", to.unit)))?;
            let from_scale = compound_to_si_scale(&from_atoms)?;
            let to_scale = compound_to_si_scale(&to_atoms)?;
            Ok(from.amount.clone() * from_scale / to_scale)
        }
    }
}

/// Canonical display abbreviation for a given abbreviation string.
/// Returns `abbrev` unchanged if not found.
pub fn canonical_display(abbrev: &str) -> &str {
    lookup_unit(abbrev).map(|u| u.display).unwrap_or(abbrev)
}

/// Convert `amount` (in `from` unit) to `to` unit using FBig arithmetic.
/// Returns `CalcError::IncompatibleUnits` if categories differ.
pub fn convert(amount: FBig, from: &Unit, to: &Unit) -> Result<FBig, CalcError> {
    if from.category != to.category {
        return Err(CalcError::IncompatibleUnits(format!(
            "cannot convert {} to {}",
            from.category.name(),
            to.category.name()
        )));
    }
    if from.abbrev == to.abbrev || from.display == to.display {
        return Ok(amount);
    }
    match from.category {
        UnitCategory::Temperature => convert_temperature(amount, from.display, to.display),
        _ => {
            let from_scale = parse_scale(from.to_base.expect("linear unit must have to_base"));
            let to_scale = parse_scale(to.to_base.expect("linear unit must have to_base"));
            Ok(amount * from_scale / to_scale)
        }
    }
}

fn convert_temperature(amount: FBig, from_display: &str, to_display: &str) -> Result<FBig, CalcError> {
    match (from_display, to_display) {
        ("°F", "°C") => Ok((amount - fbig_int(32)) * fbig_int(5) / fbig_int(9)),
        ("°C", "°F") => Ok(amount * fbig_int(9) / fbig_int(5) + fbig_int(32)),
        _ => Ok(amount),
    }
}

/// A numeric value tagged with a physical unit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaggedValue {
    /// The numeric amount in the named unit's scale.
    pub amount: FBig,
    /// Unit abbreviation (e.g. "oz", "°F"). Canonical display is looked up via `canonical_display()`.
    pub unit: String,
    /// SI dimension vector. Populated from the unit registry; used for arithmetic type-checking.
    /// `#[serde(default)]` allows old session files (no `dim` field) to deserialise without error.
    #[serde(default)]
    pub dim: DimensionVector,
}

impl TaggedValue {
    /// Create a compound-unit TaggedValue with a pre-parsed FBig amount and atom-derived metadata.
    pub fn new_compound(amount: FBig, unit: String, dim: DimensionVector) -> Self {
        Self { amount, unit, dim }
    }

    pub fn new(amount: f64, unit: impl Into<String>) -> Self {
        let unit_str = unit.into();
        // Normalise alias to canonical display string
        let display = canonical_display(&unit_str).to_string();
        let dim = lookup_unit(&display)
            .map(|u| u.dim.clone())
            .unwrap_or_default();
        Self {
            amount: FBig::try_from(amount).unwrap_or(FBig::ZERO),
            unit: display,
            dim,
        }
    }

    /// Return the static Unit definition, if the unit is known.
    pub fn unit_def(&self) -> Option<&'static Unit> {
        lookup_unit(&self.unit)
    }

    /// Convert this tagged value to a different unit abbreviation or compound unit expression.
    pub fn convert_to(&self, target_abbrev: &str) -> Result<TaggedValue, CalcError> {
        let target_display = canonical_display(target_abbrev);
        let to_simple = lookup_unit(target_display);
        let from_simple = self.unit_def();

        match (from_simple, to_simple) {
            (Some(from), Some(to)) => {
                // Both simple units — use existing convert()
                let converted = convert(self.amount.clone(), from, to)?;
                Ok(TaggedValue {
                    amount: converted,
                    unit: target_display.to_string(),
                    dim: to.dim.clone(),
                })
            }
            _ => {
                // At least one is compound — parse both as atom lists
                let from_atoms = parse_unit_expr_atoms(&self.unit)
                    .map_err(|_| CalcError::IncompatibleUnits(format!("unknown unit: {}", self.unit)))?;
                let to_atoms = parse_unit_expr_atoms(target_abbrev)
                    .or_else(|_| parse_unit_expr_atoms(target_display))
                    .map_err(|_| CalcError::InvalidInput(format!("unknown unit: {}", target_abbrev)))?;

                let from_dim = atoms_to_dim(&from_atoms);
                let to_dim = atoms_to_dim(&to_atoms);
                if from_dim != to_dim {
                    return Err(CalcError::IncompatibleUnits(format!(
                        "cannot convert {} to {}", self.unit, target_abbrev
                    )));
                }
                let from_scale = compound_to_si_scale(&from_atoms)?;
                let to_scale = compound_to_si_scale(&to_atoms)?;
                let converted = self.amount.clone() * from_scale / to_scale;
                let to_display = atoms_to_display(&to_atoms);
                Ok(TaggedValue {
                    amount: converted,
                    unit: to_display,
                    dim: to_dim,
                })
            }
        }
    }

    pub fn display(&self) -> String {
        format!("{} {}", crate::engine::value::format_fbig(&self.amount), self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build an FBig from an f64 for test inputs.
    fn fbig(v: f64) -> FBig {
        FBig::try_from(v).unwrap()
    }

    // ── lookup ───────────────────────────────────────────────────────────────

    #[test]
    fn test_lookup_known_unit() {
        assert!(lookup_unit("oz").is_some());
        assert!(lookup_unit("g").is_some());
        assert!(lookup_unit("°F").is_some());
        assert!(lookup_unit("F").is_some()); // alias
    }

    #[test]
    fn test_lookup_unknown_unit() {
        assert!(lookup_unit("fathoms").is_none());
        assert!(lookup_unit("psi").is_none());
        assert!(lookup_unit("OZ").is_none()); // case-sensitive
    }

    #[test]
    fn test_canonical_display_alias() {
        assert_eq!(canonical_display("F"), "°F");
        assert_eq!(canonical_display("C"), "°C");
        assert_eq!(canonical_display("degF"), "°F");
        assert_eq!(canonical_display("degC"), "°C");
    }

    #[test]
    fn test_canonical_display_canonical() {
        assert_eq!(canonical_display("oz"), "oz");
        assert_eq!(canonical_display("°F"), "°F");
    }

    // ── weight conversion ────────────────────────────────────────────────────

    #[test]
    fn test_oz_to_g() {
        // AC-3: 1.9 oz → ~53.86 g
        let oz = lookup_unit("oz").unwrap();
        let g = lookup_unit("g").unwrap();
        let result = convert(fbig(1.9), oz, g).unwrap();
        assert!((result.to_f64().value() - 53.8640939).abs() < 0.001,
            "1.9 oz in grams = {}", result.to_f64().value());
    }

    #[test]
    fn test_g_to_oz() {
        let g = lookup_unit("g").unwrap();
        let oz = lookup_unit("oz").unwrap();
        let result = convert(fbig(53.86), g, oz).unwrap();
        assert!((result.to_f64().value() - 1.9).abs() < 0.01,
            "53.86 g in oz = {}", result.to_f64().value());
    }

    #[test]
    fn test_lb_to_g() {
        // AC-16: 1 lb → 453.592 g
        let lb = lookup_unit("lb").unwrap();
        let g = lookup_unit("g").unwrap();
        let result = convert(fbig(1.0), lb, g).unwrap();
        assert!((result.to_f64().value() - 453.59237).abs() < 0.001,
            "1 lb in g = {}", result.to_f64().value());
    }

    #[test]
    fn test_oz_to_kg() {
        let oz = lookup_unit("oz").unwrap();
        let kg = lookup_unit("kg").unwrap();
        let result = convert(fbig(1.0), oz, kg).unwrap();
        assert!((result.to_f64().value() - 0.028349523125).abs() < 1e-9,
            "1 oz in kg = {}", result.to_f64().value());
    }

    // ── length conversion ────────────────────────────────────────────────────

    #[test]
    fn test_ft_to_m() {
        // AC-4: 6 ft → 1.8288 m
        let ft = lookup_unit("ft").unwrap();
        let m = lookup_unit("m").unwrap();
        let result = convert(fbig(6.0), ft, m).unwrap();
        assert!((result.to_f64().value() - 1.8288).abs() < 1e-9,
            "6 ft in m = {}", result.to_f64().value());
    }

    #[test]
    fn test_in_to_cm() {
        let inch = lookup_unit("in").unwrap();
        let cm = lookup_unit("cm").unwrap();
        let result = convert(fbig(1.0), inch, cm).unwrap();
        assert!((result.to_f64().value() - 2.54).abs() < 1e-9,
            "1 in in cm = {}", result.to_f64().value());
    }

    #[test]
    fn test_mi_to_km() {
        let mi = lookup_unit("mi").unwrap();
        let km = lookup_unit("km").unwrap();
        let result = convert(fbig(1.0), mi, km).unwrap();
        assert!((result.to_f64().value() - 1.609344).abs() < 1e-6,
            "1 mi in km = {}", result.to_f64().value());
    }

    // ── length conversion no noise ────────────────────────────────────────────

    #[test]
    fn test_ft_to_cm_no_noise() {
        // 1.223 ft → cm should display as 37.27704, not 37.27704000000001
        let ft = lookup_unit("ft").unwrap();
        let cm = lookup_unit("cm").unwrap();
        let result = convert(parse_scale("1.223"), ft, cm).unwrap();
        let displayed = crate::engine::value::format_fbig(&result);
        assert_eq!(displayed, "37.27704",
            "expected clean 37.27704, got {}", displayed);
    }

    #[test]
    fn test_ft_cm_ft_roundtrip_no_noise() {
        // 3.2 ft → cm → ft should round-trip cleanly
        let ft = lookup_unit("ft").unwrap();
        let cm = lookup_unit("cm").unwrap();
        let start = parse_scale("3.2");
        let in_cm = convert(start, ft, cm).unwrap();
        let back = convert(in_cm, cm, ft).unwrap();
        let displayed = crate::engine::value::format_fbig(&back);
        assert_eq!(displayed, "3.2",
            "round-trip 3.2 ft→cm→ft, got {}", displayed);
    }

    // ── temperature conversion ────────────────────────────────────────────────

    #[test]
    fn test_f_to_c() {
        // AC-5: 98.6 °F → 37 °C
        let f = lookup_unit("°F").unwrap();
        let c = lookup_unit("°C").unwrap();
        let result = convert(fbig(98.6), f, c).unwrap();
        assert!((result.to_f64().value() - 37.0).abs() < 0.001,
            "98.6 °F in °C = {}", result.to_f64().value());
    }

    #[test]
    fn test_c_to_f() {
        // AC-6: 100 °C → 212 °F
        let c = lookup_unit("°C").unwrap();
        let f = lookup_unit("°F").unwrap();
        let result = convert(fbig(100.0), c, f).unwrap();
        assert!((result.to_f64().value() - 212.0).abs() < 0.001,
            "100 °C in °F = {}", result.to_f64().value());
    }

    #[test]
    fn test_f_to_c_freezing() {
        let f = lookup_unit("°F").unwrap();
        let c = lookup_unit("°C").unwrap();
        let result = convert(fbig(32.0), f, c).unwrap();
        assert!(result.to_f64().value().abs() < 1e-9,
            "32 °F = 0 °C, got {}", result.to_f64().value());
    }

    #[test]
    fn test_temperature_alias_f_to_c() {
        // "F" alias should resolve to °F for conversion
        let tagged = TaggedValue::new(98.6, "F");
        assert_eq!(tagged.unit, "°F");
        let converted = tagged.convert_to("C").unwrap();
        assert_eq!(converted.unit, "°C");
        assert!((converted.amount.to_f64().value() - 37.0).abs() < 0.001);
    }

    // ── incompatible categories ───────────────────────────────────────────────

    #[test]
    fn test_incompatible_weight_to_length() {
        let oz = lookup_unit("oz").unwrap();
        let m = lookup_unit("m").unwrap();
        assert!(matches!(convert(fbig(1.0), oz, m), Err(CalcError::IncompatibleUnits(_))));
    }

    #[test]
    fn test_incompatible_weight_to_temperature() {
        let g = lookup_unit("g").unwrap();
        let f = lookup_unit("°F").unwrap();
        assert!(matches!(convert(fbig(1.0), g, f), Err(CalcError::IncompatibleUnits(_))));
    }

    // ── same unit (no conversion) ─────────────────────────────────────────────

    #[test]
    fn test_same_unit_no_op() {
        let oz = lookup_unit("oz").unwrap();
        let result = convert(fbig(1.9), oz, oz).unwrap();
        assert!((result.to_f64().value() - 1.9).abs() < 1e-10);
    }

    // ── TaggedValue ──────────────────────────────────────────────────────────

    #[test]
    fn test_tagged_value_new_normalises_alias() {
        let t = TaggedValue::new(98.6, "F");
        assert_eq!(t.unit, "°F");
        assert!((t.amount.to_f64().value() - 98.6).abs() < 1e-10);
    }

    #[test]
    fn test_tagged_value_display() {
        let t = TaggedValue::new(1.9, "oz");
        assert_eq!(t.display(), "1.9 oz");
    }

    #[test]
    fn test_tagged_value_convert_to() {
        let t = TaggedValue::new(1.9, "oz");
        let converted = t.convert_to("g").unwrap();
        assert_eq!(converted.unit, "g");
        assert!((converted.amount.to_f64().value() - 53.86).abs() < 0.01);
    }

    #[test]
    fn test_tagged_value_convert_to_incompatible() {
        let t = TaggedValue::new(1.9, "oz");
        assert!(matches!(t.convert_to("m"), Err(CalcError::IncompatibleUnits(_))));
    }

    #[test]
    fn test_tagged_value_serde_roundtrip() {
        let t = TaggedValue::new(1.9, "oz");
        let json = serde_json::to_string(&t).expect("serialize");
        let restored: TaggedValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(t, restored);
    }

    // ── AC-2: unit registry SI dimensions ───────────────────────────────────

    #[test]
    fn test_registry_si_dimensions() {
        let mass_dim = DimensionVector { kg: 1, ..Default::default() };
        let len_dim = DimensionVector { m: 1, ..Default::default() };
        let temp_dim = DimensionVector { k: 1, ..Default::default() };
        let time_dim = DimensionVector { s: 1, ..Default::default() };

        for abbrev in &["oz", "lb", "g", "kg"] {
            let u = lookup_unit(abbrev).unwrap_or_else(|| panic!("unit {} not found", abbrev));
            assert_eq!(u.dim, mass_dim, "{} should have mass dim", abbrev);
        }
        for abbrev in &["mm", "cm", "m", "km", "ft", "in", "yd", "mi"] {
            let u = lookup_unit(abbrev).unwrap_or_else(|| panic!("unit {} not found", abbrev));
            assert_eq!(u.dim, len_dim, "{} should have length dim", abbrev);
        }
        for abbrev in &["°F", "°C"] {
            let u = lookup_unit(abbrev).unwrap_or_else(|| panic!("unit {} not found", abbrev));
            assert_eq!(u.dim, temp_dim, "{} should have temperature dim", abbrev);
        }
        let s_unit = lookup_unit("s").expect("s not found");
        assert_eq!(s_unit.dim, time_dim);
    }

    // ── AC-3: TaggedValue serde round-trip preserves dim ────────────────────

    #[test]
    fn test_tagged_value_dim_serde_roundtrip() {
        // Simulate a compound dim (m:1, s:-1) that compound-unit-operations will produce.
        let t = TaggedValue {
            amount: fbig(27.78),
            unit: "m/s".to_string(),
            dim: DimensionVector { m: 1, s: -1, ..Default::default() },
        };
        let json = serde_json::to_string(&t).expect("serialize");
        let restored: TaggedValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.dim, t.dim);
        assert_eq!(restored.unit, "m/s");
    }

    #[test]
    fn test_tagged_value_new_populates_dim() {
        let oz = TaggedValue::new(1.9, "oz");
        assert_eq!(oz.dim, DimensionVector { kg: 1, ..Default::default() });

        let ft = TaggedValue::new(6.0, "ft");
        assert_eq!(ft.dim, DimensionVector { m: 1, ..Default::default() });

        let f = TaggedValue::new(98.6, "F");
        assert_eq!(f.dim, DimensionVector { k: 1, ..Default::default() });
    }

    #[test]
    fn test_convert_to_preserves_dim() {
        let oz = TaggedValue::new(1.9, "oz");
        let g = oz.convert_to("g").unwrap();
        assert_eq!(g.dim, DimensionVector { kg: 1, ..Default::default() });
    }

    // ── DimensionVector arithmetic ───────────────────────────────────────────

    #[test]
    fn test_dimension_vector_arithmetic() {
        let mass = DimensionVector { kg: 1, ..Default::default() };
        let accel = DimensionVector { m: 1, s: -2, ..Default::default() };

        // force = mass × acceleration: {kg:1} + {m:1, s:-2} = {kg:1, m:1, s:-2}
        let force = mass.add(&accel);
        assert_eq!(force, DimensionVector { kg: 1, m: 1, s: -2, ..Default::default() });

        // dimensionless from same-unit division
        assert!(mass.sub(&mass).is_dimensionless());

        // reciprocal
        let recip = accel.negate();
        assert_eq!(recip, DimensionVector { m: -1, s: 2, ..Default::default() });

        // sqrt of area {m:2} → {m:1}
        let area = DimensionVector { m: 2, ..Default::default() };
        assert_eq!(area.halve(), Some(DimensionVector { m: 1, ..Default::default() }));

        // sqrt of speed {m:1, s:-1} → None (odd exponent)
        let speed = DimensionVector { m: 1, s: -1, ..Default::default() };
        assert_eq!(speed.halve(), None);
    }

    // ── compound unit atom parsing ────────────────────────────────────────────

    #[test]
    fn test_parse_unit_expr_atoms_simple() {
        let atoms = parse_unit_expr_atoms("m").unwrap();
        assert_eq!(atoms, vec![("m".to_string(), 1i8)]);
    }

    #[test]
    fn test_parse_unit_expr_atoms_speed() {
        let atoms = parse_unit_expr_atoms("m/s").unwrap();
        assert_eq!(atoms, vec![("m".to_string(), 1i8), ("s".to_string(), -1i8)]);
    }

    #[test]
    fn test_parse_unit_expr_atoms_acceleration() {
        let atoms = parse_unit_expr_atoms("m/s2").unwrap();
        assert_eq!(atoms, vec![("m".to_string(), 1i8), ("s".to_string(), -2i8)]);
    }

    #[test]
    fn test_parse_unit_expr_atoms_force() {
        let atoms = parse_unit_expr_atoms("kg*m/s2").unwrap();
        assert!(atoms.contains(&("kg".to_string(), 1i8)));
        assert!(atoms.contains(&("m".to_string(), 1i8)));
        assert!(atoms.contains(&("s".to_string(), -2i8)));
    }

    #[test]
    fn test_parse_unit_expr_atoms_area() {
        let atoms = parse_unit_expr_atoms("m2").unwrap();
        assert_eq!(atoms, vec![("m".to_string(), 2i8)]);
    }

    #[test]
    fn test_parse_unit_expr_atoms_speed_with_space() {
        // space as numerator separator: "kg m/s2"
        let atoms = parse_unit_expr_atoms("kg m/s2").unwrap();
        assert!(atoms.contains(&("kg".to_string(), 1i8)));
        assert!(atoms.contains(&("m".to_string(), 1i8)));
        assert!(atoms.contains(&("s".to_string(), -2i8)));
    }

    #[test]
    fn test_parse_unit_expr_atoms_unknown_unit() {
        let result = parse_unit_expr_atoms("m/fathom2");
        assert!(matches!(result, Err(CalcError::InvalidInput(e)) if e.contains("unknown unit: fathom")));
    }

    #[test]
    fn test_parse_unit_expr_atoms_temperature_rejected() {
        assert!(parse_unit_expr_atoms("m/°C").is_err());
        assert!(parse_unit_expr_atoms("F/s").is_err());
    }

    #[test]
    fn test_parse_unit_expr_atoms_double_slash() {
        assert!(parse_unit_expr_atoms("m//s").is_err());
    }

    #[test]
    fn test_atoms_to_dim_speed() {
        let atoms = parse_unit_expr_atoms("km/h").unwrap();
        let dim = atoms_to_dim(&atoms);
        assert_eq!(dim, DimensionVector { m: 1, s: -1, ..Default::default() });
    }

    #[test]
    fn test_atoms_to_display_speed() {
        let atoms = parse_unit_expr_atoms("km/h").unwrap();
        assert_eq!(atoms_to_display(&atoms), "km/h");
    }

    #[test]
    fn test_atoms_to_display_area() {
        let atoms = parse_unit_expr_atoms("m2").unwrap();
        assert_eq!(atoms_to_display(&atoms), "m2");
    }

    #[test]
    fn test_atoms_to_display_force() {
        let atoms = vec![
            ("kg".to_string(), 1i8),
            ("m".to_string(), 1i8),
            ("s".to_string(), -2i8),
        ];
        assert_eq!(atoms_to_display(&atoms), "kg*m/s2");
    }

    #[test]
    fn test_combine_atoms_mul_cancellation() {
        // km/h * h = km
        let y = parse_unit_expr_atoms("km/h").unwrap();
        let x = parse_unit_expr_atoms("h").unwrap();
        let result = combine_atoms_mul(&y, &x);
        assert_eq!(result, vec![("km".to_string(), 1i8)]);
        assert_eq!(atoms_to_display(&result), "km");
    }

    #[test]
    fn test_combine_atoms_mul_area() {
        // m * m = m2
        let y = parse_unit_expr_atoms("m").unwrap();
        let x = parse_unit_expr_atoms("m").unwrap();
        let result = combine_atoms_mul(&y, &x);
        assert_eq!(result, vec![("m".to_string(), 2i8)]);
    }

    #[test]
    fn test_compound_to_si_scale_speed() {
        // km/h → 1000/3600 m/s
        let atoms = parse_unit_expr_atoms("km/h").unwrap();
        let scale = compound_to_si_scale(&atoms).unwrap();
        let expected = 1000.0_f64 / 3600.0;
        assert!((scale.to_f64().value() - expected).abs() < 1e-9,
            "km/h scale = {}", scale.to_f64().value());
    }

    // ── AC-14: compound unit conversion ──────────────────────────────────────

    #[test]
    fn test_convert_tagged_compound_ms_to_kmh() {
        // 27.78 m/s → km/h ≈ 100
        let tv = TaggedValue {
            amount: FBig::try_from(27.78_f64).unwrap(),
            unit: "m/s".to_string(),
            dim: DimensionVector { m: 1, s: -1, ..Default::default() },
        };
        let converted = tv.convert_to("km/h").unwrap();
        assert_eq!(converted.unit, "km/h");
        assert!((converted.amount.to_f64().value() - 100.0).abs() < 0.1,
            "27.78 m/s in km/h = {}", converted.amount.to_f64().value());
    }

    #[test]
    fn test_convert_tagged_compound_incompatible() {
        let tv = TaggedValue {
            amount: FBig::try_from(1.0_f64).unwrap(),
            unit: "m/s".to_string(),
            dim: DimensionVector { m: 1, s: -1, ..Default::default() },
        };
        assert!(matches!(tv.convert_to("kg"), Err(CalcError::IncompatibleUnits(_))));
    }

    // ── unit alias helpers ────────────────────────────────────────────────────

    #[test]
    fn test_aliases_for_dim_force() {
        // force dimension: kg*m*s⁻² → N should be returned
        let force_dim = DimensionVector { kg: 1, m: 1, s: -2, ..Default::default() };
        let aliases = aliases_for_dim(&force_dim);
        assert!(aliases.contains(&"N"), "N should match force dimension, got: {:?}", aliases);
    }

    #[test]
    fn test_aliases_for_dim_speed() {
        // speed dimension: m*s⁻¹ → kph should be returned
        let speed_dim = DimensionVector { m: 1, s: -1, ..Default::default() };
        let aliases = aliases_for_dim(&speed_dim);
        assert!(aliases.contains(&"kph"), "kph should match speed dimension, got: {:?}", aliases);
    }

    #[test]
    fn test_aliases_for_dim_no_match() {
        // pure length dimension has no alias
        let length_dim = DimensionVector { m: 1, ..Default::default() };
        let aliases = aliases_for_dim(&length_dim);
        assert!(aliases.is_empty(), "no alias should match plain length, got: {:?}", aliases);
    }
}
