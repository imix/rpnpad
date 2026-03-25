use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::engine::stack::CalcState;
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
    ("d", "drop"),
    ("p", "dup"),
    ("r", "rot"),
    ("u", "undo"),
    ("y", "yank"),
    ("S", "store"),
];

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

const CHORD_LEADERS: &[(&str, &str)] = &[
    ("t", "trig"),
    ("l", "log"),
    ("f", "√"),
    ("c", "const"),
    ("m", "mode"),
    ("x", "base"),
    ("X", "hex"),
];

const UNARY_OPS: &[(&str, &str)] = &[("!", "fact"), ("n", "neg"), ("q", "x²"), ("w", "√")];

const CHORD_LEADERS_DEPTH0: &[(&str, &str)] =
    &[("c", "const"), ("m", "mode"), ("x", "base"), ("X", "hex")];

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

    if matches!(mode, AppMode::Insert(_)) {
        let lines = vec![
            Line::raw("Enter  push"),
            Line::raw("Esc    cancel"),
            Line::raw("Bksp   delete"),
            Line::raw(""),
            Line::raw("+  add    -  sub"),
            Line::raw("*  mul    /  div"),
            Line::raw("^  pow    !  fact"),
            Line::raw("%  mod    n  neg"),
            Line::raw("q  x²    w  √"),
            Line::raw("s  swap   d  drop"),
            Line::raw("p  dup    r  rot"),
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
        let (header, ops): (&str, &[(&str, &str)]) = match category {
            ChordCategory::Trig => ("[TRIG]", TRIG_OPS),
            ChordCategory::Log => ("[LOG]", LOG_OPS),
            ChordCategory::Functions => ("[FN]", FN_OPS),
            ChordCategory::Constants => ("[CONST]", CONST_OPS),
            ChordCategory::AngleMode => ("[MODE]", ANGLE_OPS),
            ChordCategory::Base => ("[BASE]", BASE_OPS),
            ChordCategory::HexStyle => ("[HEX]", HEX_STYLE_OPS),
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
}
