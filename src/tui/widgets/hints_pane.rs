use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::engine::base::Base;
use crate::engine::stack::CalcState;
use crate::engine::units::{aliases_for_dim, UnitCategory};
use crate::engine::value::CalcValue;
use crate::input::mode::{AppMode, ChordCategory};

const ARITHMETIC: &[(&str, &str)] = &[
    ("+", "add"),
    ("-", "sub"),
    ("*", "mul"),
    ("/", "div"),
    ("^", "pow"),
    ("%", "mod"),
    ("!", "fact"),
    ("n", "neg"),
    ("q", "x²"),
    ("w", "√"),
];

const STACK_OPS: &[(&str, &str)] = &[
    ("s", "swap"),
    ("Bksp", "drop"),
    ("p", "dup"),
    ("R", "rot"),
    ("u", "undo"),
    ("y", "yank"),
    ("S", "store"),
];

const UNIT_OPS: &[(&str, &str)] = &[("U", "convert")];

const SESSION_OPS: &[(&str, &str)] = &[("Q", "quit")];

const TRIG_OPS: &[(&str, &str)] = &[
    ("s", "sin"),
    ("c", "cos"),
    ("a", "tan"),
    ("S", "asin"),
    ("C", "acos"),
    ("A", "atan"),
];

const LOG_OPS: &[(&str, &str)] = &[("l", "ln"), ("L", "log10"), ("e", "exp"), ("E", "exp10")];

const FN_OPS: &[(&str, &str)] = &[("s", "sqrt"), ("q", "sq"), ("r", "recip"), ("a", "abs")];

const CONST_OPS: &[(&str, &str)] = &[("p", "π"), ("e", "e"), ("g", "φ")];

const ANGLE_OPS: &[(&str, &str)] = &[("d", "deg"), ("r", "rad"), ("g", "grad")];

const BASE_OPS: &[(&str, &str)] = &[("c", "dec"), ("h", "hex"), ("o", "oct"), ("b", "bin")];

const HEX_STYLE_OPS: &[(&str, &str)] = &[("c", "0xFF"), ("a", "$FF"), ("s", "#FF"), ("i", "FFh")];

const ROUNDING_OPS: &[(&str, &str)] = &[
    ("f", "⌊x⌋"),
    ("c", "⌈x⌉"),
    ("t", "trunc"),
    ("r", "RND↓"),
    ("s", "sgn"),
];

const CHORD_LEADERS: &[(&str, &str)] = &[
    ("t", "trig"),
    ("l", "log"),
    ("f", "√"),
    ("r", "round"),
    ("c", "const"),
    ("C", "config"),
];

const UNARY_OPS: &[(&str, &str)] = &[("!", "fact"), ("n", "neg"), ("q", "x²"), ("w", "√")];

const CHORD_LEADERS_DEPTH0: &[(&str, &str)] = &[("c", "const"), ("C", "config")];

fn entries_to_lines(entries: &[(&str, &str)]) -> Vec<Line<'static>> {
    entries
        .chunks(2)
        .map(|chunk| {
            let left = format!("{:<2} {:<6}", chunk[0].0, chunk[0].1);
            let right = chunk
                .get(1)
                .map(|(k, l)| format!("  {:<2} {:<6}", k, l))
                .unwrap_or_default();
            Line::raw(format!("{}{}", left, right))
        })
        .collect()
}

fn chord_leaders_to_lines(leaders: &[(&str, &str)]) -> Vec<Line<'static>> {
    leaders
        .chunks(2)
        .map(|chunk| {
            let left = format!("{}›  {:<5}", chunk[0].0, chunk[0].1);
            let right = chunk
                .get(1)
                .map(|(k, l)| format!("  {}›  {:<5}", k, l))
                .unwrap_or_default();
            Line::raw(format!("{}{}", left, right))
        })
        .collect()
}

fn registers_to_lines(state: &CalcState, max_width: usize) -> Vec<Line<'static>> {
    let mut entries: Vec<_> = state.registers.iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    entries
        .into_iter()
        .map(|(name, val)| {
            let val_str = val.display_with_base(state.base);
            let line = format!("{name}  {val_str}  {name} RCL");
            let truncated: String = line.chars().take(max_width).collect();
            Line::raw(truncated)
        })
        .collect()
}

pub fn render(f: &mut Frame, area: Rect, mode: &AppMode, state: &CalcState) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Hints")
        .title_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let area = inner;
    let dim = Style::default().add_modifier(Modifier::DIM);

    if matches!(mode, AppMode::AlphaStore(_)) {
        let lines = vec![
            Line::styled("STORE NAME", dim),
            Line::raw(""),
            Line::raw("Enter  store"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::PrecisionInput(_)) {
        let lines = vec![
            Line::styled("PRECISION", dim),
            Line::raw(""),
            Line::raw("Enter  confirm (1–15)"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::Browse(_)) {
        let lines = vec![
            Line::styled("BROWSE", dim),
            Line::raw(""),
            Line::raw("Enter  roll to top"),
            Line::raw("Esc    cancel"),
            Line::raw("↑      deeper"),
            Line::raw("↓      toward top"),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::ConvertInput(_)) {
        // Determine what is at the stack top: simple unit category, compound unit, or nothing.
        let (category, compound_unit) = match state.stack.last() {
            Some(CalcValue::Tagged(tv)) => {
                let cat = tv.unit_def().map(|u| u.category);
                let compound = if cat.is_none() { Some(tv.unit.as_str()) } else { None };
                (cat, compound)
            }
            _ => (None, None),
        };
        let mut lines = vec![
            Line::styled("CONVERT TO UNIT", dim),
            Line::raw(""),
            Line::raw("Enter  convert"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
            Line::raw(""),
        ];
        if let Some(unit_str) = compound_unit {
            // Compound unit: valid targets are open-ended (same dimension vector).
            // Show source unit as context and prompt for a compatible expression.
            lines.push(Line::styled("COMPOUND UNIT", dim));
            lines.push(Line::raw(format!("source: {}", unit_str)));
            lines.push(Line::raw(""));
            lines.push(Line::styled("enter target unit expression", dim));
        } else {
            match category {
                Some(UnitCategory::Weight) | None => {
                    lines.push(Line::styled("WEIGHT", dim));
                    lines.push(Line::raw("g  kg  lb  oz"));
                }
                _ => {}
            }
            match category {
                Some(UnitCategory::Length) | None => {
                    if category.is_none() { lines.push(Line::raw("")); }
                    lines.push(Line::styled("LENGTH", dim));
                    lines.push(Line::raw("cm  ft  in  km"));
                    lines.push(Line::raw("m   mi  mm  yd"));
                }
                _ => {}
            }
            match category {
                Some(UnitCategory::Temperature) | None => {
                    if category.is_none() { lines.push(Line::raw("")); }
                    lines.push(Line::styled("TEMPERATURE", dim));
                    lines.push(Line::raw("°C  °F"));
                    lines.push(Line::styled("also: C  F  degC  degF", dim));
                }
                _ => {}
            }
        }
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::Insert(_)) {
        let lines = vec![
            Line::raw("Enter  push"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
            Line::raw(""),
            Line::styled("unit: 1.9 oz  6 ft  98.6 F  (or 1 m/s)", dim),
            Line::raw(""),
            Line::raw("+  add    -  sub"),
            Line::raw("*  mul    /  div"),
            Line::raw("^  pow    !  fact"),
            Line::raw("%  mod    n  neg"),
            Line::raw("q  x²    w  √"),
            Line::raw("s  swap   Bksp drop"),
            Line::raw("p  dup    R  rot"),
            Line::raw(""),
            Line::raw("Q  quit"),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::InsertUnit(_)) {
        let lines = vec![
            Line::raw("Enter  push"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
            Line::raw(""),
            Line::styled("unit expression — all keys literal", dim),
            Line::styled("examples: m/s  kg*m/s2  km/h", dim),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if matches!(mode, AppMode::Alpha(_)) {
        let lines = vec![
            Line::raw("Enter  submit"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
            Line::raw(""),
            Line::styled("all chars literal", dim),
        ];
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    if let AppMode::Chord(category) = mode {
        if matches!(category, ChordCategory::Config) {
            let mut lines: Vec<Line<'static>> = vec![
                Line::styled("[CONFIG]", dim),
                Line::raw(""),
                Line::raw("ANGLE  d deg  r rad  g grad"),
                Line::raw("BASE   c dec  h hex  o oct  b bin"),
                Line::raw("NOTE   f fix  s sci  a auto"),
                Line::raw("PREC   p set"),
            ];
            if state.base == Base::Hex {
                lines.push(Line::raw("HEX    1 0xFF 2 $FF 3 #FF 4 FFh"));
            }
            f.render_widget(Paragraph::new(lines), area);
            return;
        }
        let (header, ops): (&str, &[(&str, &str)]) = match category {
            ChordCategory::Trig => ("[TRIG]", TRIG_OPS),
            ChordCategory::Log => ("[LOG]", LOG_OPS),
            ChordCategory::Functions => ("[FN]", FN_OPS),
            ChordCategory::Constants => ("[CONST]", CONST_OPS),
            ChordCategory::AngleMode => ("[MODE]", ANGLE_OPS),
            ChordCategory::Base => ("[BASE]", BASE_OPS),
            ChordCategory::HexStyle => ("[HEX]", HEX_STYLE_OPS),
            ChordCategory::Rounding => ("[ROUND]", ROUNDING_OPS),
            ChordCategory::Config => unreachable!("handled above"),
        };
        let mut lines: Vec<Line<'static>> = vec![Line::styled(header, dim)];
        lines.extend(entries_to_lines(ops));
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    let depth = state.stack.len();
    let mut lines: Vec<Line<'static>> = Vec::new();

    if depth >= 2 {
        lines.push(Line::styled("ARITHMETIC", dim));
        lines.extend(entries_to_lines(ARITHMETIC));
        lines.push(Line::raw(""));
    } else if depth == 1 {
        lines.push(Line::styled("ARITHMETIC", dim));
        lines.extend(entries_to_lines(UNARY_OPS));
        lines.push(Line::raw(""));
    }

    lines.push(Line::styled("STACK", dim));
    lines.extend(entries_to_lines(STACK_OPS));
    lines.push(Line::raw(""));

    if depth == 0 {
        lines.extend(chord_leaders_to_lines(CHORD_LEADERS_DEPTH0));
    } else {
        lines.extend(chord_leaders_to_lines(CHORD_LEADERS));
    }

    let top_is_tagged = state.stack.last().map(|v| matches!(v, CalcValue::Tagged(_))).unwrap_or(false);
    if top_is_tagged {
        lines.push(Line::raw(""));
        lines.push(Line::styled("UNITS", dim));
        lines.extend(entries_to_lines(UNIT_OPS));
        if let Some(CalcValue::Tagged(tv)) = state.stack.last() {
            for alias in aliases_for_dim(&tv.dim) {
                lines.push(Line::raw(format!("  → {}", alias)));
            }
        }
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled("SESSION", dim));
    lines.extend(entries_to_lines(SESSION_OPS));

    if !state.registers.is_empty() {
        lines.push(Line::raw(""));
        lines.push(Line::styled("REGISTERS", dim));
        lines.extend(registers_to_lines(state, area.width as usize));
    }

    f.render_widget(Paragraph::new(lines), area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::stack::CalcState;
    use crate::engine::value::CalcValue;
    use ratatui::{backend::TestBackend, Terminal};

    fn state_with_depth(n: usize) -> CalcState {
        let mut s = CalcState::new();
        for i in 0..n {
            s.stack.push(CalcValue::from_f64(i as f64 + 1.0));
        }
        s
    }

    fn render_hints(
        mode: AppMode,
        state: CalcState,
        width: u16,
        height: u16,
    ) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| render(f, f.area(), &mode, &state))
            .unwrap();
        terminal.backend().buffer().clone()
    }

    fn full_content(buf: &ratatui::buffer::Buffer) -> String {
        let area = buf.area();
        (0..area.height)
            .flat_map(|y| (0..area.width).map(move |x| (x, y)))
            .map(|(x, y)| buf.cell((x, y)).unwrap().symbol().to_string())
            .collect()
    }

    #[test]
    fn test_normal_mode_shows_arithmetic_header() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("ARITHMETIC"));
    }

    #[test]
    fn test_normal_mode_shows_stack_header() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("STACK"));
    }

    #[test]
    fn test_normal_mode_shows_chord_leaders() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains('›'));
    }

    #[test]
    fn test_normal_mode_shows_add_op() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains('+'));
        assert!(content.contains("add"));
    }

    #[test]
    fn test_insert_mode_shows_push_hint() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(content.contains("push"));
        assert!(!content.contains("ARITHMETIC"));
    }

    #[test]
    fn test_insert_mode_shows_cancel_hint() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(content.contains("cancel"));
    }

    #[test]
    fn test_narrow_pane_no_panic() {
        // Just verify it doesn't panic with a very small area
        let _ = render_hints(AppMode::Normal, CalcState::new(), 5, 3);
    }

    #[test]
    fn test_chord_trig_shows_header() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Trig),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(content.contains("[TRIG]"));
    }

    #[test]
    fn test_chord_trig_shows_sin() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Trig),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(content.contains("sin"));
    }

    #[test]
    fn test_chord_trig_hides_arithmetic_header() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Trig),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(!content.contains("ARITHMETIC"));
    }

    #[test]
    fn test_chord_base_shows_hex() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Base),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(content.contains("hex"));
    }

    // ── Depth-filtering tests (Story 3.4) ────────────────────────────────────

    #[test]
    fn test_depth0_hides_arithmetic_header() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("ARITHMETIC"));
    }

    #[test]
    fn test_depth0_hides_binary_ops() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains('+'));
        assert!(!content.contains("add"));
    }

    #[test]
    fn test_depth0_hides_unary_ops() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("fact"));
        assert!(!content.contains("neg"));
    }

    #[test]
    fn test_depth0_shows_constants_leader() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("const"));
    }

    #[test]
    fn test_depth0_shows_stack_ops() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("STACK"));
    }

    #[test]
    fn test_depth0_hides_trig_leader() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("trig"));
    }

    #[test]
    fn test_depth1_shows_unary_ops() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("fact"));
        assert!(content.contains("neg"));
    }

    #[test]
    fn test_depth1_hides_binary_ops() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("add"));
        assert!(!content.contains("sub"));
    }

    #[test]
    fn test_depth1_shows_all_chord_leaders() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("trig"));
    }

    #[test]
    fn test_depth2_shows_full_arithmetic() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("add"));
        assert!(content.contains("mul"));
    }

    // ── Register section tests (Story 3.5) ───────────────────────────────────

    #[test]
    fn test_no_registers_hides_section() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("REGISTERS"));
    }

    #[test]
    fn test_registers_shows_section_header() {
        let mut s = CalcState::new();
        s.registers
            .insert("r1".to_string(), CalcValue::from_f64(3.14));
        let buf = render_hints(AppMode::Normal, s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("REGISTERS"));
    }

    #[test]
    fn test_registers_shows_register_name() {
        let mut s = CalcState::new();
        s.registers
            .insert("r1".to_string(), CalcValue::from_f64(3.14));
        let buf = render_hints(AppMode::Normal, s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("r1"));
    }

    #[test]
    fn test_registers_shows_recall_command() {
        let mut s = CalcState::new();
        s.registers
            .insert("r1".to_string(), CalcValue::from_f64(1.0));
        let buf = render_hints(AppMode::Normal, s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("r1 RCL"));
    }

    #[test]
    fn test_registers_not_shown_in_insert_mode() {
        let mut s = CalcState::new();
        s.registers
            .insert("r1".to_string(), CalcValue::from_f64(1.0));
        let buf = render_hints(AppMode::Insert(String::new()), s, 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("REGISTERS"));
    }

    #[test]
    fn test_multiple_registers_all_shown() {
        let mut s = CalcState::new();
        s.registers
            .insert("aa".to_string(), CalcValue::from_f64(1.0));
        s.registers
            .insert("bb".to_string(), CalcValue::from_f64(2.0));
        let buf = render_hints(AppMode::Normal, s, 40, 25);
        let content = full_content(&buf);
        assert!(content.contains("aa"));
        assert!(content.contains("bb"));
    }

    // ── direct-common-functions: AC-3 / AC-5 ────────────────────────────────

    // AC-3: depth≥2 → x² appears directly in Normal hints
    #[test]
    fn test_depth2_shows_square_directly() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("x²"), "depth≥2 hints should show x² directly");
    }

    // AC-3: depth≥2 → √ appears directly in Normal hints
    #[test]
    fn test_depth2_shows_sqrt_directly() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains('√'), "depth≥2 hints should show √ directly");
    }

    // AC-3: depth==1 → x² and √ appear in unary hints
    #[test]
    fn test_depth1_shows_square_and_sqrt_directly() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("x²"), "depth==1 hints should show x²");
        assert!(content.contains('√'), "depth==1 hints should show √");
    }

    // AC-3: chord leader for f shows √ (not fn)
    #[test]
    fn test_chord_leader_f_shows_sqrt_symbol() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains('√'), "f› chord leader should display √");
        assert!(!content.contains("fn"), "f› chord leader should not display opaque 'fn'");
    }

    // AC-5: recip (1/x) not shown directly in Normal-mode hints at depth≥1
    #[test]
    fn test_normal_hints_no_recip() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 20);
        let content = full_content(&buf);
        assert!(!content.contains("recip"), "recip should not appear in Normal hints directly");
    }

    // AC-5: abs not shown directly in Normal-mode hints at depth≥1
    #[test]
    fn test_normal_hints_no_abs() {
        let buf = render_hints(AppMode::Normal, state_with_depth(2), 40, 20);
        let content = full_content(&buf);
        // "abs" does not appear directly (only inside the f› chord submenu)
        // but "ARITHMETIC" contains "add" not "abs" — safe check on mode not being Chord
        assert!(!content.contains("abs"), "abs should not appear in Normal hints directly");
    }

    // Insert mode hints show q/w shortcuts
    #[test]
    fn test_insert_hints_show_square_and_sqrt() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("x²"), "Insert mode hints should show x²");
        assert!(content.contains('√'), "Insert mode hints should show √");
    }

    // ── Story 4.1: Named Memory Registers ────────────────────────────────────

    // AC 5: normal mode shows S and "store" in STACK section
    #[test]
    fn test_normal_mode_shows_store_hint() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(
            content.contains('S'),
            "S key should appear in normal mode hints"
        );
        assert!(
            content.contains("store"),
            "store label should appear in normal mode hints"
        );
    }

    // Insert mode shows InsertSubmitThen bindings (all 12 op shortcuts)
    #[test]
    fn test_insert_mode_shows_submit_then_bindings() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("add"), "Insert mode should show 'add' hint");
        assert!(content.contains("sub"), "Insert mode should show 'sub' hint");
        assert!(content.contains("mul"), "Insert mode should show 'mul' hint");
        assert!(content.contains("div"), "Insert mode should show 'div' hint");
        assert!(content.contains("mod"), "Insert mode should show 'mod' hint");
        assert!(content.contains("dup"), "Insert mode should show 'dup' hint");
    }

    // Insert mode still shows push/cancel/delete
    #[test]
    fn test_insert_mode_still_shows_push_cancel() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("push"));
        assert!(content.contains("cancel"));
    }

    // AC-7/AC-8: InsertUnit mode shows unit expression hint, no op shortcuts
    #[test]
    fn test_insert_unit_mode_hints() {
        let buf = render_hints(AppMode::InsertUnit("1 m".into()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(content.contains("push"), "InsertUnit should show push hint");
        assert!(content.contains("cancel"), "InsertUnit should show cancel hint");
        assert!(content.contains("literal"), "InsertUnit should mention literal keys");
        assert!(!content.contains("add"), "InsertUnit should NOT show op shortcuts");
        assert!(!content.contains("div"), "InsertUnit should NOT show div shortcut");
    }

    // Alpha mode shows submit/cancel/delete (no op shortcuts)
    #[test]
    fn test_alpha_mode_shows_submit_cancel() {
        let buf = render_hints(AppMode::Alpha(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("submit"));
        assert!(content.contains("cancel"));
        assert!(!content.contains("add"), "Alpha mode should NOT show op shortcuts");
    }

    // AC 6: AlphaStore mode shows STORE NAME header and store prompt
    #[test]
    fn test_alpha_store_mode_shows_store_prompt() {
        let buf = render_hints(AppMode::AlphaStore(String::new()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(
            content.contains("store"),
            "AlphaStore mode should show 'store' action"
        );
        assert!(
            content.contains("cancel"),
            "AlphaStore mode should show 'cancel' action"
        );
        assert!(
            content.contains("delete"),
            "AlphaStore mode should show 'delete' action"
        );
    }

    // AC 6: AlphaStore mode does NOT show arithmetic or stack ops
    #[test]
    fn test_alpha_store_hides_normal_hints() {
        let buf = render_hints(AppMode::AlphaStore(String::new()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(!content.contains("ARITHMETIC"));
        assert!(!content.contains("STACK"));
    }

    // AC-9: Browse mode shows navigation hint panel
    #[test]
    fn test_browse_mode_shows_hints() {
        let buf = render_hints(AppMode::Browse(2), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(content.contains("roll"), "Browse mode should show 'roll to top' hint");
        assert!(content.contains("cancel"), "Browse mode should show 'cancel' hint");
        assert!(!content.contains("ARITHMETIC"), "Browse mode should not show normal mode hints");
        assert!(!content.contains("STACK"), "Browse mode should not show stack ops section");
    }

    // ── apply-rounding-and-sign-ops ──────────────────────────────────────────

    // AC-10: r› chord leader appears in Normal hints at depth ≥ 1
    #[test]
    fn test_depth1_shows_rounding_chord_leader() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("round"), "r› chord leader should show 'round' at depth≥1");
    }

    // AC-10: Q quit appears in Normal mode hints (now in SESSION section)
    #[test]
    fn test_normal_mode_shows_quit_hint() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 25);
        let content = full_content(&buf);
        assert!(content.contains('Q'), "Q key should appear in normal mode hints");
        assert!(content.contains("quit"), "quit label should appear in normal mode hints");
    }

    // AC-5: Q quit is in SESSION section, not grouped with STACK operations
    #[test]
    fn test_quit_in_session_section_not_stack() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 25);
        let content = full_content(&buf);
        assert!(content.contains("SESSION"), "SESSION section should appear in normal mode");
        // SESSION must appear after STACK in the rendered output
        let stack_pos = content.find("STACK").expect("STACK section must exist");
        let session_pos = content.find("SESSION").expect("SESSION section must exist");
        assert!(
            session_pos > stack_pos,
            "SESSION section should appear after STACK section"
        );
    }

    // Rounding chord submenu renders with [ROUND] header
    #[test]
    fn test_rounding_chord_shows_header() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Rounding),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(content.contains("[ROUND]"), "Rounding chord should show [ROUND] header");
    }

    // Rounding chord submenu shows floor/ceil entries
    #[test]
    fn test_rounding_chord_shows_ops() {
        let buf = render_hints(
            AppMode::Chord(ChordCategory::Rounding),
            CalcState::new(),
            40,
            10,
        );
        let content = full_content(&buf);
        assert!(content.contains("trunc"), "Rounding chord should show trunc");
        assert!(content.contains("sgn"), "Rounding chord should show sgn");
    }

    // r› not shown at depth 0 (all rounding ops need stack items)
    #[test]
    fn test_depth0_hides_rounding_chord_leader() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(!content.contains("round"), "r› chord should not show at depth 0");
    }

    // AlphaStore mode does NOT show register section
    #[test]
    fn test_alpha_store_hides_registers_section() {
        let mut s = CalcState::new();
        s.registers
            .insert("r1".to_string(), CalcValue::from_f64(1.0));
        let buf = render_hints(AppMode::AlphaStore(String::new()), s, 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("REGISTERS"));
    }

    // ── configure-settings-chord AC-13 ─────────────────────────────────────

    // AC-13: Config chord shows [CONFIG] header
    #[test]
    fn test_config_chord_shows_header() {
        let buf = render_hints(AppMode::Chord(ChordCategory::Config), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("[CONFIG]"), "Config chord should show [CONFIG]: {:?}", content);
    }

    // AC-13: Config chord shows ANGLE, BASE, NOTE, PREC sections
    #[test]
    fn test_config_chord_shows_all_categories() {
        let buf = render_hints(AppMode::Chord(ChordCategory::Config), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("ANGLE"), "should show ANGLE: {:?}", content);
        assert!(content.contains("BASE"), "should show BASE: {:?}", content);
        assert!(content.contains("NOTE"), "should show NOTE: {:?}", content);
        assert!(content.contains("PREC"), "should show PREC: {:?}", content);
    }

    // AC-13: Config chord HEX STYLE only shown when base=HEX
    #[test]
    fn test_config_chord_hex_style_shown_only_in_hex() {
        // non-HEX: no HEX section
        let buf = render_hints(AppMode::Chord(ChordCategory::Config), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(!content.contains("HEX"), "HEX section should be hidden when base is not HEX: {:?}", content);

        // HEX base: HEX section visible
        let mut s = CalcState::new();
        s.base = crate::engine::base::Base::Hex;
        let buf = render_hints(AppMode::Chord(ChordCategory::Config), s, 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("HEX"), "HEX section should appear when base is HEX: {:?}", content);
    }

    // AC-13: C› config chord leader shown in Normal mode hints
    #[test]
    fn test_normal_mode_shows_config_chord_leader() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains('C'), "C› chord leader should appear in normal mode hints");
        assert!(content.contains("config"), "config label should appear");
    }

    // AC-12: m and x no longer appear as chord leaders in Normal mode hints
    #[test]
    fn test_normal_mode_no_m_x_chord_leaders() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(!content.contains("m›"), "m› should be removed from hints");
        assert!(!content.contains("x›"), "x› should be removed from hints");
    }

    // PrecisionInput mode shows PRECISION hint
    #[test]
    fn test_precision_input_mode_shows_hint() {
        let buf = render_hints(AppMode::PrecisionInput(String::new()), CalcState::new(), 40, 10);
        let content = full_content(&buf);
        assert!(content.contains("PRECISION"), "PRECISION mode should show hint: {:?}", content);
    }

    // ── unit-aware-values AC-23: ConvertInput hint panel ─────────────────────

    // AC-23: ConvertInput panel shows CONVERT TO UNIT header
    #[test]
    fn test_convert_input_mode_shows_header() {
        let buf = render_hints(AppMode::ConvertInput(String::new()), CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("CONVERT TO UNIT"), "ConvertInput panel should show header: {:?}", content);
    }

    // AC-23: ConvertInput panel shows key table (Enter/Esc/Bksp)
    #[test]
    fn test_convert_input_mode_shows_key_table() {
        let buf = render_hints(AppMode::ConvertInput(String::new()), CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("convert"), "should show 'convert' action: {:?}", content);
        assert!(content.contains("cancel"), "should show 'cancel' action: {:?}", content);
        assert!(content.contains("delete"), "should show 'delete' action: {:?}", content);
    }

    // AC-23: ConvertInput shows only WEIGHT group when top is a weight value
    #[test]
    fn test_convert_input_filters_to_weight() {
        use crate::engine::units::TaggedValue;
        let mut s = CalcState::new();
        s.stack.push(CalcValue::Tagged(TaggedValue::new(1.9, "oz")));
        let buf = render_hints(AppMode::ConvertInput(String::new()), s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("WEIGHT"), "should show WEIGHT: {:?}", content);
        assert!(content.contains("oz"), "should show oz: {:?}", content);
        assert!(!content.contains("LENGTH"), "should NOT show LENGTH: {:?}", content);
        assert!(!content.contains("TEMPERATURE"), "should NOT show TEMPERATURE: {:?}", content);
    }

    // AC-23: ConvertInput shows only LENGTH group when top is a length value
    #[test]
    fn test_convert_input_filters_to_length() {
        use crate::engine::units::TaggedValue;
        let mut s = CalcState::new();
        s.stack.push(CalcValue::Tagged(TaggedValue::new(6.0, "ft")));
        let buf = render_hints(AppMode::ConvertInput(String::new()), s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("LENGTH"), "should show LENGTH: {:?}", content);
        assert!(content.contains("ft"), "should show ft: {:?}", content);
        assert!(!content.contains("WEIGHT"), "should NOT show WEIGHT: {:?}", content);
        assert!(!content.contains("TEMPERATURE"), "should NOT show TEMPERATURE: {:?}", content);
    }

    // AC-23: ConvertInput shows only TEMPERATURE group when top is a temperature value
    #[test]
    fn test_convert_input_filters_to_temperature() {
        use crate::engine::units::TaggedValue;
        let mut s = CalcState::new();
        s.stack.push(CalcValue::Tagged(TaggedValue::new(98.6, "F")));
        let buf = render_hints(AppMode::ConvertInput(String::new()), s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("TEMPERATURE"), "should show TEMPERATURE: {:?}", content);
        assert!(content.contains("degC") || content.contains("degF"), "should show aliases: {:?}", content);
        assert!(!content.contains("WEIGHT"), "should NOT show WEIGHT: {:?}", content);
        assert!(!content.contains("LENGTH"), "should NOT show LENGTH: {:?}", content);
    }

    // AC-23: ConvertInput shows all groups when stack is empty (no category context)
    #[test]
    fn test_convert_input_shows_all_when_no_context() {
        let buf = render_hints(AppMode::ConvertInput(String::new()), CalcState::new(), 40, 25);
        let content = full_content(&buf);
        assert!(content.contains("WEIGHT"), "should show WEIGHT with no context: {:?}", content);
        assert!(content.contains("LENGTH"), "should show LENGTH with no context: {:?}", content);
        assert!(content.contains("TEMPERATURE"), "should show TEMPERATURE with no context: {:?}", content);
    }

    // AC-23: ConvertInput shows COMPOUND UNIT section when top is a compound unit
    #[test]
    fn test_convert_input_compound_unit_shows_compound_section() {
        use crate::engine::units::{TaggedValue, DimensionVector};
        use dashu::float::FBig;
        let mut s = CalcState::new();
        // km/h: m¹ s⁻¹
        let tv = TaggedValue::new_compound(
            FBig::try_from(50.0_f64).unwrap(),
            "km/h".to_string(),
            DimensionVector { m: 1, s: -1, ..Default::default() },
        );
        s.stack.push(CalcValue::Tagged(tv));
        let buf = render_hints(AppMode::ConvertInput(String::new()), s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("COMPOUND UNIT"), "should show COMPOUND UNIT: {:?}", content);
        assert!(content.contains("km/h"), "should show source unit: {:?}", content);
        assert!(!content.contains("WEIGHT"), "should NOT show WEIGHT: {:?}", content);
        assert!(!content.contains("LENGTH"), "should NOT show LENGTH: {:?}", content);
        assert!(!content.contains("TEMPERATURE"), "should NOT show TEMPERATURE: {:?}", content);
    }

    // AC-23: ConvertInput compound section shows prompt text
    #[test]
    fn test_convert_input_compound_unit_shows_prompt() {
        use crate::engine::units::{TaggedValue, DimensionVector};
        use dashu::float::FBig;
        let mut s = CalcState::new();
        let tv = TaggedValue::new_compound(
            FBig::try_from(9.8_f64).unwrap(),
            "m/s2".to_string(),
            DimensionVector { m: 1, s: -2, ..Default::default() },
        );
        s.stack.push(CalcValue::Tagged(tv));
        let buf = render_hints(AppMode::ConvertInput(String::new()), s, 40, 20);
        let content = full_content(&buf);
        assert!(content.contains("m/s2"), "should show source unit: {:?}", content);
        assert!(content.contains("enter target"), "should show prompt: {:?}", content);
    }

    // AC-23: ConvertInput panel does NOT show normal mode sections
    #[test]
    fn test_convert_input_mode_hides_normal_sections() {
        let buf = render_hints(AppMode::ConvertInput(String::new()), CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(!content.contains("ARITHMETIC"), "should not show ARITHMETIC: {:?}", content);
        assert!(!content.contains("STACK"), "should not show STACK: {:?}", content);
    }

    // ── unit-aware-values AC-24: UNITS section visibility ────────────────────

    // AC-24: UNITS section shown when stack top is tagged
    #[test]
    fn test_units_section_shown_when_top_is_tagged() {
        use crate::engine::units::TaggedValue;
        let mut s = CalcState::new();
        s.stack.push(CalcValue::Tagged(TaggedValue::new(1.9, "oz")));
        let buf = render_hints(AppMode::Normal, s, 40, 30);
        let content = full_content(&buf);
        assert!(content.contains("UNITS"), "UNITS section should appear when top is tagged: {:?}", content);
        assert!(content.contains('U'), "U key should appear in UNITS section: {:?}", content);
        assert!(content.contains("convert"), "convert label should appear: {:?}", content);
    }

    // AC-24: UNITS section absent when stack top is plain number
    #[test]
    fn test_units_section_absent_when_top_is_plain() {
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 25);
        let content = full_content(&buf);
        assert!(!content.contains("UNITS"), "UNITS section should be absent for plain stack: {:?}", content);
    }

    // AC-24: UNITS section absent when stack is empty
    #[test]
    fn test_units_section_absent_when_stack_empty() {
        let buf = render_hints(AppMode::Normal, CalcState::new(), 40, 20);
        let content = full_content(&buf);
        assert!(!content.contains("UNITS"), "UNITS section should be absent when stack is empty: {:?}", content);
    }

    // U key no longer appears in STACK section
    #[test]
    fn test_u_key_not_in_stack_section() {
        // With a plain stack, U should not appear at all (UNITS section hidden)
        let buf = render_hints(AppMode::Normal, state_with_depth(1), 40, 25);
        let content = full_content(&buf);
        assert!(!content.contains("UNITS"), "no UNITS section for plain value: {:?}", content);
    }

    // ── unit-aliases AC-5: alias names in UNITS section ──────────────────────

    // AC-5: force-dimensioned stack top → "N" appears in UNITS section as a conversion target
    #[test]
    fn test_units_section_shows_alias_for_force_dim() {
        use crate::engine::units::{DimensionVector, TaggedValue};
        use dashu::float::FBig;
        let mut s = CalcState::new();
        // Push a TaggedValue with force dimension (kg*m/s2) — same as N
        let tv = TaggedValue::new_compound(
            FBig::try_from(9.8_f64).unwrap(),
            "kg*m/s2".to_string(),
            DimensionVector { kg: 1, m: 1, s: -2, ..Default::default() },
        );
        s.stack.push(CalcValue::Tagged(tv));
        let buf = render_hints(AppMode::Normal, s, 60, 25);
        let content = full_content(&buf);
        assert!(content.contains("→ N"), "UNITS section should show '→ N' for force dim, got: {:?}", content);
    }

    // Speed dimension → kph appears in UNITS section
    #[test]
    fn test_units_section_shows_alias_for_speed_dim() {
        use crate::engine::units::{DimensionVector, TaggedValue};
        use dashu::float::FBig;
        let mut s = CalcState::new();
        let tv = TaggedValue::new_compound(
            FBig::try_from(100.0_f64).unwrap(),
            "km/h".to_string(),
            DimensionVector { m: 1, s: -1, ..Default::default() },
        );
        s.stack.push(CalcValue::Tagged(tv));
        let buf = render_hints(AppMode::Normal, s, 60, 25);
        let content = full_content(&buf);
        assert!(content.contains("→ kph"), "UNITS section should show '→ kph' for speed dim, got: {:?}", content);
    }

    // Plain length value → no alias shown
    #[test]
    fn test_units_section_no_alias_for_length_dim() {
        use crate::engine::units::TaggedValue;
        let mut s = CalcState::new();
        s.stack.push(CalcValue::Tagged(TaggedValue::new(6.0, "ft")));
        let buf = render_hints(AppMode::Normal, s, 60, 25);
        let content = full_content(&buf);
        assert!(!content.contains("→ N"), "no N alias for length: {:?}", content);
        assert!(!content.contains("→ kph"), "no kph alias for length: {:?}", content);
    }

    // ── unit-aware-values AC-25: Insert mode unit syntax hint ─────────────────

    // AC-25: Insert mode hints show unit input syntax example
    #[test]
    fn test_insert_mode_shows_unit_syntax_hint() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("oz") || content.contains("ft") || content.contains(" F"),
            "Insert mode should show unit syntax example: {:?}", content);
    }

    // AC-25: unit syntax hint is dim/contextual, does not replace op shortcuts
    #[test]
    fn test_insert_mode_unit_hint_alongside_ops() {
        let buf = render_hints(AppMode::Insert(String::new()), CalcState::new(), 40, 15);
        let content = full_content(&buf);
        assert!(content.contains("add"), "op shortcuts still present: {:?}", content);
        assert!(content.contains("oz") || content.contains("ft"),
            "unit hint also present: {:?}", content);
    }
}
