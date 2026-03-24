use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::input::mode::AppMode;

pub fn render(f: &mut Frame, area: Rect, mode: &AppMode) {
    let text = match mode {
        AppMode::Insert(buf) | AppMode::Alpha(buf) | AppMode::AlphaStore(buf) => {
            format!("> {}_", buf)
        }
        _ => "> ".to_string(),
    };
    f.render_widget(Paragraph::new(text), area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::mode::AppMode;
    use ratatui::{backend::TestBackend, Terminal};

    fn render_input_line(mode: &AppMode, width: u16) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, f.area(), mode)).unwrap();
        terminal.backend().buffer().clone()
    }

    fn row_content(buf: &ratatui::buffer::Buffer, row: u16) -> String {
        let width = buf.area().width;
        (0..width)
            .map(|x| buf.cell((x, row)).unwrap().symbol().to_string())
            .collect()
    }

    // AC 6: normal mode shows prompt only
    #[test]
    fn test_normal_mode_shows_prompt() {
        let buf = render_input_line(&AppMode::Normal, 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> "),
            "normal mode should show '> ': {:?}",
            content
        );
    }

    // Insert mode shows buffer content and cursor
    #[test]
    fn test_insert_mode_shows_buffer_and_cursor() {
        let buf = render_input_line(&AppMode::Insert("42".into()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> 42_"),
            "Insert mode should show '> 42_': {:?}",
            content
        );
    }

    // Insert mode with empty buffer shows just cursor
    #[test]
    fn test_insert_empty_buffer_shows_cursor() {
        let buf = render_input_line(&AppMode::Insert(String::new()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> _"),
            "empty Insert buffer should show '> _': {:?}",
            content
        );
    }

    // Alpha mode shows buffer content and cursor
    #[test]
    fn test_alpha_mode_shows_buffer_and_cursor() {
        let buf = render_input_line(&AppMode::Alpha("r1 RCL".into()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> r1 RCL_"),
            "Alpha mode should show '> r1 RCL_': {:?}",
            content
        );
    }

    // Alpha mode with empty buffer shows just cursor
    #[test]
    fn test_alpha_empty_buffer_shows_cursor() {
        let buf = render_input_line(&AppMode::Alpha(String::new()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> _"),
            "empty Alpha buffer should show '> _': {:?}",
            content
        );
    }

    // AC 6: AlphaStore mode shows register name buffer and cursor
    #[test]
    fn test_alpha_store_shows_buffer_and_cursor() {
        let buf = render_input_line(&AppMode::AlphaStore("my".into()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> my_"),
            "AlphaStore mode should show '> my_': {:?}",
            content
        );
    }

    // AC 6: AlphaStore with empty buffer shows just cursor
    #[test]
    fn test_alpha_store_empty_shows_cursor() {
        let buf = render_input_line(&AppMode::AlphaStore(String::new()), 20);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("> _"),
            "empty AlphaStore buffer should show '> _': {:?}",
            content
        );
    }
}
