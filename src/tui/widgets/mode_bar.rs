use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::engine::{
    base::{Base, HexStyle},
    stack::CalcState,
};
use crate::input::mode::AppMode;

pub fn render(f: &mut Frame, area: Rect, mode: &AppMode, state: &CalcState) {
    let mode_str = match mode {
        AppMode::Normal | AppMode::Chord(_) => "[NORMAL]",
        AppMode::Insert(_) | AppMode::AlphaStore(_) => "[INSERT]",
        AppMode::Alpha(_) => "[ALPHA]",
    };

    let right_str = if state.base == Base::Hex {
        let hex_example = match state.hex_style {
            HexStyle::ZeroX => "0xFF",
            HexStyle::Dollar => "$FF",
            HexStyle::Hash => "#FF",
            HexStyle::Suffix => "FFh",
        };
        format!("{}  {}  {}", state.angle_mode, state.base, hex_example)
    } else {
        format!("{}  {}", state.angle_mode, state.base)
    };

    let total_content = mode_str.len() + right_str.len();
    let pad_len = (area.width as usize).saturating_sub(total_content);
    let padding = " ".repeat(pad_len);

    let style = Style::default().fg(Color::Yellow);
    let line = Line::from(vec![
        Span::styled(mode_str, style),
        Span::raw(padding),
        Span::styled(right_str, style),
    ]);

    f.render_widget(Paragraph::new(line), area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{
        angle::AngleMode,
        base::{Base, HexStyle},
        stack::CalcState,
    };
    use crate::input::mode::{AppMode, ChordCategory};
    use ratatui::{backend::TestBackend, Terminal};

    fn render_mode_bar(mode: &AppMode, state: &CalcState, width: u16) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, f.area(), mode, state)).unwrap();
        terminal.backend().buffer().clone()
    }

    fn row_content(buf: &ratatui::buffer::Buffer, row: u16) -> String {
        let width = buf.area().width;
        (0..width)
            .map(|x| buf.cell((x, row)).unwrap().symbol().to_string())
            .collect()
    }

    // AC 1: normal mode shows [NORMAL]
    #[test]
    fn test_normal_mode_shows_normal() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[NORMAL]"),
            "normal mode should show '[NORMAL]': {:?}",
            content
        );
    }

    // Insert mode shows [INSERT]
    #[test]
    fn test_insert_mode_shows_insert() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Insert(String::new()), &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[INSERT]"),
            "Insert mode should show '[INSERT]': {:?}",
            content
        );
    }

    // Alpha mode shows [ALPHA]
    #[test]
    fn test_alpha_mode_shows_alpha() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Alpha(String::new()), &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[ALPHA]"),
            "Alpha mode should show '[ALPHA]': {:?}",
            content
        );
    }

    // AC 6: AlphaStore mode shows [INSERT]
    #[test]
    fn test_alpha_store_mode_shows_insert() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::AlphaStore(String::new()), &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[INSERT]"),
            "AlphaStore mode should show '[INSERT]': {:?}",
            content
        );
    }

    // Chord mode should show [NORMAL] (chord is a transient sub-state of normal)
    #[test]
    fn test_chord_mode_shows_normal() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Chord(ChordCategory::Trig), &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[NORMAL]"),
            "chord mode should show '[NORMAL]': {:?}",
            content
        );
    }

    // AC 1: angle mode appears on right
    #[test]
    fn test_angle_deg_appears() {
        let mut state = CalcState::new();
        state.angle_mode = AngleMode::Deg;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("DEG"),
            "DEG angle mode should appear: {:?}",
            content
        );
    }

    // AC 1: base appears on right
    #[test]
    fn test_base_dec_appears() {
        let mut state = CalcState::new();
        state.base = Base::Dec;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("DEC"),
            "DEC base should appear: {:?}",
            content
        );
    }

    // AC 3: HEX base shows both HEX and hex style example
    #[test]
    fn test_hex_base_shows_style() {
        let mut state = CalcState::new();
        state.base = Base::Hex;
        state.hex_style = HexStyle::ZeroX;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("HEX"),
            "HEX base should appear when in hex mode: {:?}",
            content
        );
        assert!(
            content.contains("0xFF"),
            "HexStyle::ZeroX should show '0xFF': {:?}",
            content
        );
    }

    // AC 3: all HexStyle variants display correctly
    #[test]
    fn test_hex_style_variants() {
        let styles = [
            (HexStyle::ZeroX, "0xFF"),
            (HexStyle::Dollar, "$FF"),
            (HexStyle::Hash, "#FF"),
            (HexStyle::Suffix, "FFh"),
        ];
        for (style, expected) in &styles {
            let mut state = CalcState::new();
            state.base = Base::Hex;
            state.hex_style = *style;
            let buf = render_mode_bar(&AppMode::Normal, &state, 40);
            let content = row_content(&buf, 0);
            assert!(
                content.contains(expected),
                "HexStyle {:?} should show '{}': {:?}",
                style,
                expected,
                content
            );
        }
    }

    // AC 3: non-HEX base does NOT show any hex style indicator
    #[test]
    fn test_non_hex_no_style_suffix() {
        let mut state = CalcState::new();
        state.base = Base::Dec;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            !content.contains("0xFF")
                && !content.contains("$FF")
                && !content.contains("#FF")
                && !content.contains("FFh"),
            "DEC mode should show no hex style indicator: {:?}",
            content
        );
    }

    // Mode bar text is styled yellow
    #[test]
    fn test_mode_bar_is_yellow() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let cell = buf.cell((0u16, 0u16)).unwrap();
        assert_eq!(cell.fg, Color::Yellow, "mode bar text should be Yellow");
    }
}
