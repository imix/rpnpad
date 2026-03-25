use ratatui::{
    layout::{
        Constraint::{Length, Min, Percentage},
        Layout,
    },
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::input::mode::AppMode;
use crate::tui::{
    app::App,
    widgets::{error_line, hints_pane, input_line, mode_bar, stack_pane},
};

/// Maximum content width — prevents the layout from sprawling on wide terminals.
pub(crate) const MAX_WIDTH: u16 = 100;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // Center the content within MAX_WIDTH; use full width if terminal is narrower.
    let content_area = if area.width > MAX_WIDTH {
        Layout::horizontal([Min(0), Length(MAX_WIDTH), Min(0)]).split(area)[1]
    } else {
        area
    };

    // Guard: need at least 7 rows (2 for border + 4 fixed rows + 1 for content).
    // Check before rendering anything to avoid drawing a border into a too-small area.
    if content_area.height < 7 {
        return;
    }

    // Draw border around the entire calculator and shrink into the inner area.
    let title_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let border = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" rpncalc ")
        .title_style(title_style);
    let inner_area = border.inner(content_area);
    f.render_widget(border, content_area);

    let width = inner_area.width;
    // 4 fixed rows: input, error, separator, mode bar
    let outer =
        Layout::vertical([Min(0), Length(1), Length(1), Length(1), Length(1)]).split(inner_area);

    // width is derived from inner_area (post-border, post-centering), so this threshold is always
    // evaluated against the capped content column width, not the raw terminal width.
    let browse_cursor = if let AppMode::Browse(pos) = &app.mode {
        Some(*pos)
    } else {
        None
    };

    if width < 60 {
        stack_pane::render(f, outer[0], &app.state, browse_cursor);
    } else {
        let inner = Layout::horizontal([Percentage(50), Percentage(50)]).split(outer[0]);
        stack_pane::render(f, inner[0], &app.state, browse_cursor);
        hints_pane::render(f, inner[1], &app.mode, &app.state);
    }

    input_line::render(f, outer[1], &app.mode);
    error_line::render(f, outer[2], app.error_message.as_deref());
    // Separator between content area and mode bar
    f.render_widget(Block::default().borders(Borders::TOP), outer[3]);
    mode_bar::render(f, outer[4], &app.mode, &app.state, app.last_command.as_deref());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::App;
    use ratatui::{backend::TestBackend, Terminal};

    fn render_layout(width: u16, height: u16) -> ratatui::buffer::Buffer {
        let app = App::new();
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, &app)).unwrap();
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
    fn test_narrow_terminal_hides_hints() {
        let buf = render_layout(50, 20);
        let content = full_content(&buf);
        // "STACK" is the hints_pane section header (uppercase). The stack_pane block title is
        // "Stack" (mixed-case), so "STACK" in the flat buffer uniquely signals hints_pane output.
        assert!(!content.contains("STACK"));
    }

    #[test]
    fn test_wide_terminal_shows_hints() {
        let buf = render_layout(80, 20);
        let content = full_content(&buf);
        // App::new() starts in Normal mode — confirmed by mode bar showing "[NORMAL]".
        assert!(
            content.contains("[NORMAL]"),
            "app should start in Normal mode"
        );
        assert!(content.contains("STACK"));
    }

    #[test]
    fn test_medium_terminal_shows_hints() {
        // 60-79 range: hints pane present but narrowed
        let buf = render_layout(70, 20);
        let content = full_content(&buf);
        // App::new() starts in Normal mode — confirmed by mode bar showing "[NORMAL]".
        assert!(
            content.contains("[NORMAL]"),
            "app should start in Normal mode"
        );
        assert!(content.contains("STACK"));
    }

    #[test]
    fn test_minimum_dimensions_no_panic() {
        let _ = render_layout(1, 1);
    }

    #[test]
    fn test_tiny_terminal_renders_nothing() {
        // height=6 < 7 minimum: guard fires before any rendering, buffer stays blank.
        let buf = render_layout(80, 6);
        let content = full_content(&buf);
        assert!(
            content.chars().all(|c| c == ' '),
            "terminal below minimum height should render nothing"
        );
    }

    #[test]
    fn test_fixed_rows_always_present() {
        // mode bar renders "[NORMAL]" — always present regardless of terminal size
        let buf = render_layout(80, 10);
        let content = full_content(&buf);
        assert!(content.contains("[NORMAL]"));
    }

    #[test]
    fn test_border_present() {
        // 80 cols < MAX_WIDTH so content_area starts at col 0 (no centering offset).
        // BorderType::Rounded corner chars: ╭ (top-left), ╮ (top-right), ╰ (bot-left), ╯ (bot-right).
        let buf = render_layout(80, 20);
        assert_eq!(
            buf.cell((0u16, 0u16)).unwrap().symbol(),
            "╭",
            "top-left corner"
        );
        assert_eq!(
            buf.cell((79u16, 0u16)).unwrap().symbol(),
            "╮",
            "top-right corner"
        );
        assert_eq!(
            buf.cell((0u16, 19u16)).unwrap().symbol(),
            "╰",
            "bottom-left corner"
        );
        assert_eq!(
            buf.cell((79u16, 19u16)).unwrap().symbol(),
            "╯",
            "bottom-right corner"
        );
    }

    #[test]
    fn test_outer_border_has_app_title() {
        let buf = render_layout(80, 20);
        let content = full_content(&buf);
        assert!(
            content.contains("rpncalc"),
            "outer border should contain app title 'rpncalc': {:?}",
            &content[..80.min(content.len())]
        );
    }

    #[test]
    fn test_separator_row_present() {
        // With 80×20: outer border rows 0 and 19. Inner rows 1–18.
        // Fixed rows: input=row16, error=row17, separator=row18? No:
        // Layout: Min(0)=rows1-14, input=15, error=16, separator=17, mode=18, outer-bottom=19.
        // Separator row (outer[3]) draws a TOP border — a '─' across the inner width.
        // Check that '─' appears somewhere in the buffer (it also appears in borders, but
        // the separator adds a full row of them at a specific vertical position).
        let buf = render_layout(80, 20);
        let area = buf.area();
        // The separator is outer[3] = row 18 in a 20-row buffer (rows 1-18 are inner, last is mode bar).
        // Actually: inner_area rows 1..18, layout [Min, 1, 1, 1, 1] → row18=separator area top.
        // A Block with Borders::TOP renders '─' at the top of its 1-row area.
        // Row 18 (0-indexed) = the separator row.
        let separator_row = area.height - 3; // mode bar at height-2, separator at height-3
        let row_content: String = (1..area.width - 1)
            .map(|x| buf.cell((x, separator_row)).unwrap().symbol().to_string())
            .collect();
        assert!(
            row_content.contains('─'),
            "separator row should contain '─' characters, got: {:?}",
            row_content
        );
    }

    #[test]
    fn test_minimum_split_boundary() {
        // width=62: outer border takes 2 cols → inner_area.width=60 → each pane=30 cols.
        // Both panes should render without panic and show expected content.
        let buf = render_layout(62, 20);
        let content = full_content(&buf);
        assert!(content.contains("[NORMAL]"));
        // hints pane still visible (width >= 60)
        assert!(content.contains("STACK"));
    }

    #[test]
    fn test_split_pane_order() {
        // Stack pane is left (inner[0]), hints pane is right (inner[1]).
        // At 80×20: outer border → inner_area cols 1..79 (width=78). Each pane ≈39 cols.
        // "Stack" (stack block title) should appear in the left half (col < 40).
        // "STACK" (hints section header) should appear in the right half (col >= 40).
        let buf = render_layout(80, 20);
        let area = buf.area();
        let midpoint = area.width / 2;
        let stack_title_in_left = (0..area.height).any(|y| {
            let row: String = (0..midpoint)
                .map(|x| buf.cell((x, y)).unwrap().symbol().to_string())
                .collect();
            row.contains("Stack")
        });
        let hints_header_in_right = (0..area.height).any(|y| {
            let row: String = (midpoint..area.width)
                .map(|x| buf.cell((x, y)).unwrap().symbol().to_string())
                .collect();
            row.contains("STACK")
        });
        assert!(
            stack_title_in_left,
            "stack pane title should be in left half"
        );
        assert!(
            hints_header_in_right,
            "hints pane STACK header should be in right half"
        );
    }

    #[test]
    fn test_wide_terminal_still_shows_hints() {
        // 200-column terminal: content capped at MAX_WIDTH but hints still rendered
        let buf = render_layout(200, 20);
        let content = full_content(&buf);
        assert!(content.contains("STACK"));
        assert!(content.contains("[NORMAL]"));
    }

    #[test]
    fn test_wide_terminal_has_margins() {
        // 200-column terminal: margins appear as spaces on either side of content.
        // ratatui distributes excess space equally among Min(0) segments, so with
        // 200-col terminal and 100-col content each margin is 50 cols wide.
        // Col 10 is well inside the left margin (cols 0–49).
        // Col 190 is well inside the right margin (cols 150–199).
        let buf = render_layout(200, 20);
        let left_margin_cell = buf.cell((10u16, 10u16)).unwrap().symbol().to_string();
        assert_eq!(left_margin_cell, " ", "left margin should be blank space");
        let right_margin_cell = buf.cell((190u16, 10u16)).unwrap().symbol().to_string();
        assert_eq!(right_margin_cell, " ", "right margin should be blank space");
    }
}
