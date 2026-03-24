use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph},
    Frame,
};

use crate::engine::stack::CalcState;

pub fn render(f: &mut Frame, area: Rect, state: &CalcState, precision: usize) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Stack")
        .title_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let height = inner.height as usize;
    let width = inner.width as usize;
    let depth = state.stack.len();

    // Label width based on height so empty-stack labels align with value-row labels.
    let label_col_width = format!("{}:", height).len();

    let val_col_width = width.saturating_sub(label_col_width + 1);

    let visible_slice = if depth > height {
        &state.stack[(depth - height)..]
    } else {
        &state.stack[..]
    };

    let visible_count = visible_slice.len();
    let mut lines: Vec<Line> = Vec::with_capacity(height);

    // Empty rows at the top show position labels (HP48 style) with no value beside them.
    let blank_count = height.saturating_sub(visible_count);
    for i in 0..blank_count {
        let position = height - i;
        let label_span = Span::styled(
            format!("{:>lw$}: ", position, lw = label_col_width - 1),
            Style::default().add_modifier(Modifier::DIM),
        );
        lines.push(Line::from(vec![label_span]));
    }

    // Value rows: oldest-visible (top) → newest/X (bottom)
    for (i, val) in visible_slice.iter().enumerate() {
        let position_from_bottom = visible_count - 1 - i;

        let label = format!("{}", position_from_bottom + 1);

        let label_span = Span::styled(
            format!("{:>lw$}: ", label, lw = label_col_width - 1),
            Style::default().add_modifier(Modifier::DIM),
        );

        let val_str = val.display_with_precision(state.base, precision);
        let char_count = val_str.chars().count();
        let val_display = if char_count > val_col_width {
            let truncated: String = val_str
                .chars()
                .take(val_col_width.saturating_sub(1))
                .collect();
            format!("{}…", truncated)
        } else {
            format!("{:>width$}", val_str, width = val_col_width)
        };

        let val_style = if position_from_bottom == 0 {
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan)
        } else {
            Style::default()
        };

        lines.push(Line::from(vec![
            label_span,
            Span::styled(val_display, val_style),
        ]));
    }

    f.render_widget(Paragraph::new(lines), inner);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{base::Base, stack::CalcState, value::CalcValue};
    use dashu::integer::IBig;
    use ratatui::{backend::TestBackend, Terminal};

    fn render_pane(state: &CalcState, width: u16, height: u16) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, f.area(), state, 15)).unwrap();
        terminal.backend().buffer().clone()
    }

    fn push_int(state: &mut CalcState, n: i64) {
        state.push(CalcValue::Integer(IBig::from(n)));
    }

    fn row_content(buf: &ratatui::buffer::Buffer, row: u16) -> String {
        let width = buf.area().width;
        (0..width)
            .map(|x| buf.cell((x, row)).unwrap().symbol().to_string())
            .collect()
    }

    // AC 6: empty stack → position labels visible in all inner rows, no values beside them
    #[test]
    fn test_empty_stack_blank_rows() {
        let state = CalcState::new();
        // 20×6: border at rows 0 and 5, inner rows 1–4 (height=4)
        // Labels: 4:, 3:, 2:, 1: from top to bottom
        let buf = render_pane(&state, 20, 6);
        let row1 = row_content(&buf, 1);
        let row4 = row_content(&buf, 4);
        assert!(
            row1.contains("4:"),
            "top empty row should show label '4:', got: {:?}",
            row1
        );
        assert!(
            row4.contains("1:"),
            "bottom empty row should show label '1:', got: {:?}",
            row4
        );
    }

    // AC 1, 2: single value → 1: label at bottom, value visible
    #[test]
    fn test_x_label_single_value() {
        let mut state = CalcState::new();
        push_int(&mut state, 42);
        // 20×4: inner height 2; blank row at terminal row 1, row 1 at terminal row 2
        let buf = render_pane(&state, 20, 4);
        let bottom_row = row_content(&buf, 2);
        assert!(
            bottom_row.contains("1:"),
            "bottom row should have '1:': {:?}",
            bottom_row
        );
        assert!(
            bottom_row.contains("42"),
            "bottom row should have value '42': {:?}",
            bottom_row
        );
    }

    // AC 2: four values → 1/2/3/4 labels (HP48 convention, bottom = 1)
    #[test]
    fn test_xyzt_labels() {
        let mut state = CalcState::new();
        push_int(&mut state, 1);
        push_int(&mut state, 2);
        push_int(&mut state, 3);
        push_int(&mut state, 4);
        // 20×8: border rows 0 and 7, inner rows 1–6 (inner height 6)
        // depth=4 ≤ height=6: 2 blank rows then 4:/3:/2:/1: at rows 3/4/5/6
        let buf = render_pane(&state, 20, 8);
        let bottom_row = 6u16;
        assert!(
            row_content(&buf, bottom_row).contains("1:"),
            "row {} should have '1:'",
            bottom_row
        );
        assert!(
            row_content(&buf, bottom_row - 1).contains("2:"),
            "row {} should have '2:'",
            bottom_row - 1
        );
        assert!(
            row_content(&buf, bottom_row - 2).contains("3:"),
            "row {} should have '3:'",
            bottom_row - 2
        );
        assert!(
            row_content(&buf, bottom_row - 3).contains("4:"),
            "row {} should have '4:'",
            bottom_row - 3
        );
    }

    // AC 2: 5th entry from bottom → numeric label "5:"
    #[test]
    fn test_numeric_labels_beyond_t() {
        let mut state = CalcState::new();
        for i in 1..=5 {
            push_int(&mut state, i);
        }
        // 20×8: inner height 6, depth=5 → 1 blank at row 1, values at rows 2–6
        // i=0 (value 1, position_from_bottom=4 → "5:") at terminal row 2
        let buf = render_pane(&state, 20, 8);
        let row_5_label = row_content(&buf, 2);
        assert!(
            row_5_label.contains("5:"),
            "5th-from-bottom row should show '5:': {:?}",
            row_5_label
        );
    }

    // AC 6: X row value is Color::Cyan + Modifier::BOLD
    #[test]
    fn test_x_row_is_cyan_bold() {
        let mut state = CalcState::new();
        push_int(&mut state, 99);
        // 20×4: border(1) + padding(1) = inner.x=2; label "1: "=3 chars → value at col 5
        let buf = render_pane(&state, 20, 4);
        let cell = buf.cell((5u16, 2u16)).unwrap();
        assert_eq!(cell.fg, Color::Cyan, "X value span should be Cyan");
        assert!(
            cell.modifier.contains(Modifier::BOLD),
            "X value span should be BOLD"
        );
    }

    // AC 6: non-X rows have default style (not Cyan)
    #[test]
    fn test_older_rows_not_styled() {
        let mut state = CalcState::new();
        push_int(&mut state, 10);
        push_int(&mut state, 20);
        // 20×4: inner height 2; Y at row 1, X at row 2
        // border(1) + padding(1) = inner.x=2; label "2: "=3 chars → value at col 5
        let buf = render_pane(&state, 20, 4);
        let cell = buf.cell((5u16, 1u16)).unwrap();
        assert_ne!(cell.fg, Color::Cyan, "Y value span should NOT be Cyan");
    }

    // AC 3: value wider than column → truncated with '…', leading digits preserved
    #[test]
    fn test_truncation_long_value() {
        let mut state = CalcState::new();
        // 20-digit integer
        let big = IBig::from(12345678901234567890u64);
        state.push(CalcValue::Integer(big));
        // 12×4: border(2) + padding(2) = inner width=8; label "1: "=3 chars, val_col_width=5
        // "12345678901234567890" (20 chars) > 5 → truncated to "1234…"
        let buf = render_pane(&state, 12, 4);
        let bottom = row_content(&buf, 2);
        assert!(
            bottom.contains('…'),
            "long value should be truncated with '…': {:?}",
            bottom
        );
        assert!(
            bottom.contains("1234"),
            "truncated value should start with leading digits: {:?}",
            bottom
        );
    }

    // AC 4: stack deeper than visible → oldest entries scroll off, pane is full
    #[test]
    fn test_scroll_cuts_old_entries() {
        let mut state = CalcState::new();
        // 20×8: inner height=6; push 8 values (height+2)
        for i in 1..=8 {
            push_int(&mut state, i);
        }
        let buf = render_pane(&state, 20, 8);
        // depth=8 > height=6: no blank rows, all inner rows have a label
        for row in 1..=6u16 {
            let content = row_content(&buf, row);
            assert!(
                content.contains(':'),
                "row {} should have a label (no blank rows): {:?}",
                row,
                content
            );
        }
        // visible = last 6 values (3–8); topmost = position_from_bottom=5 → label "6:"
        let top_row = row_content(&buf, 1);
        assert!(
            top_row.contains("6:"),
            "topmost visible row should show '6:': {:?}",
            top_row
        );
    }

    // AC 1: values rendered using active base (Hex)
    #[test]
    fn test_value_uses_active_base() {
        let mut state = CalcState::new();
        state.base = Base::Hex;
        push_int(&mut state, 255);
        // display_with_base(Hex) for 255 → "0xFF"
        let buf = render_pane(&state, 20, 4);
        let bottom = row_content(&buf, 2);
        assert!(
            bottom.contains("FF"),
            "hex base should display 255 as 0xFF: {:?}",
            bottom
        );
        assert!(
            !bottom.contains("255"),
            "should not show decimal '255' in hex mode: {:?}",
            bottom
        );
    }

    // FBig gotcha guard: floats must render as decimal, never as binary FBig format
    #[test]
    fn test_float_value_displays_as_decimal() {
        let mut state = CalcState::new();
        // CalcValue::from_f64 wraps into FBig; display_with_base must use to_f64().value()
        state.push(CalcValue::from_f64(3.14));
        // 20×4: X row at terminal row 2
        let buf = render_pane(&state, 20, 4);
        let bottom = row_content(&buf, 2);
        // FBig::to_string() produces "1.57 × 2^1" — must never appear
        assert!(
            !bottom.contains('×'),
            "float must not render as binary FBig format: {:?}",
            bottom
        );
        // display_with_base(Dec) for 3.14 should produce something starting with "3.1"
        assert!(
            bottom.contains("3.1"),
            "float 3.14 should display as decimal starting with '3.1': {:?}",
            bottom
        );
    }
}
