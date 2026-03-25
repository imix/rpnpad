use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::engine::{
    base::{Base, HexStyle},
    notation::Notation,
    stack::CalcState,
};
use crate::input::mode::AppMode;

pub fn render(
    f: &mut Frame,
    area: Rect,
    mode: &AppMode,
    state: &CalcState,
    last_command: Option<&str>,
) {
    let mode_str = match mode {
        AppMode::Normal | AppMode::Chord(_) => "[NORMAL]",
        AppMode::Insert(_) | AppMode::AlphaStore(_) => "[INSERT]",
        AppMode::Alpha(_) => "[ALPHA]",
        AppMode::Browse(_) => "[BROWSE]",
        AppMode::PrecisionInput(_) => "[PREC]",
    };

    let notation_str = match state.notation {
        Notation::Fixed => "",
        Notation::Sci => "  SCI",
        Notation::Auto => "  AUTO",
    };

    let right_str = if state.base == Base::Hex {
        let hex_example = match state.hex_style {
            HexStyle::ZeroX => "0xFF",
            HexStyle::Dollar => "$FF",
            HexStyle::Hash => "#FF",
            HexStyle::Suffix => "FFh",
        };
        format!("{}  {}  {}{}", state.angle_mode, state.base, hex_example, notation_str)
    } else {
        format!("{}  {}{}", state.angle_mode, state.base, notation_str)
    };

    let style = Style::default().fg(Color::Yellow);
    let width = area.width as usize;

    // Show centre label only when there is enough room to avoid overlapping left or right.
    // Minimum layout: "<left>  <centre>  <right>" — 2 spaces on each side of centre.
    let show_centre = last_command.map_or(false, |cmd| {
        mode_str.len() + 2 + cmd.len() + 2 + right_str.len() <= width
    });

    let line = if show_centre {
        let cmd = last_command.unwrap();
        let sides_len = mode_str.len() + right_str.len() + cmd.len();
        let gap = width.saturating_sub(sides_len);
        let left_pad = gap / 2;
        let right_pad = gap - left_pad;
        Line::from(vec![
            Span::styled(mode_str, style),
            Span::raw(" ".repeat(left_pad)),
            Span::styled(cmd, style),
            Span::raw(" ".repeat(right_pad)),
            Span::styled(right_str, style),
        ])
    } else {
        let pad_len = width.saturating_sub(mode_str.len() + right_str.len());
        Line::from(vec![
            Span::styled(mode_str, style),
            Span::raw(" ".repeat(pad_len)),
            Span::styled(right_str, style),
        ])
    };

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
        render_mode_bar_with_cmd(mode, state, width, None)
    }

    fn render_mode_bar_with_cmd(
        mode: &AppMode,
        state: &CalcState,
        width: u16,
        last_command: Option<&str>,
    ) -> ratatui::buffer::Buffer {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| render(f, f.area(), mode, state, last_command))
            .unwrap();
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

    // AC-8: Browse mode shows [BROWSE]
    #[test]
    fn test_browse_mode_shows_browse() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Browse(2), &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[BROWSE]"),
            "browse mode should show '[BROWSE]': {:?}",
            content
        );
    }

    // AC-6: session start — no last command shows blank centre
    #[test]
    fn test_session_start_centre_blank() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(
            content.contains("[NORMAL]"),
            "mode indicator present: {:?}",
            content
        );
        assert!(
            !content.contains("→"),
            "no last-command arrow when centre is blank: {:?}",
            content
        );
    }

    // AC-1: single-key op label appears in centre
    #[test]
    fn test_single_op_label_shown() {
        let state = CalcState::new();
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 40, Some("+ → add"));
        let content = row_content(&buf, 0);
        assert!(
            content.contains("+ → add"),
            "last-command label should appear: {:?}",
            content
        );
    }

    // AC-2: chord label appears in centre
    #[test]
    fn test_chord_label_shown() {
        let state = CalcState::new();
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 40, Some("rf → floor"));
        let content = row_content(&buf, 0);
        assert!(
            content.contains("rf → floor"),
            "chord label should appear: {:?}",
            content
        );
    }

    // AC-5: undo label
    #[test]
    fn test_undo_label_shown() {
        let state = CalcState::new();
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 40, Some("u → undo"));
        let content = row_content(&buf, 0);
        assert!(
            content.contains("u → undo"),
            "undo label should appear: {:?}",
            content
        );
    }

    // AC-9: yank label
    #[test]
    fn test_yank_label_shown() {
        let state = CalcState::new();
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 40, Some("y → copy"));
        let content = row_content(&buf, 0);
        assert!(
            content.contains("y → copy"),
            "yank label should appear: {:?}",
            content
        );
    }

    // AC-10: mode-change chord label
    #[test]
    fn test_mode_change_chord_label_shown() {
        let state = CalcState::new();
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 40, Some("md → deg"));
        let content = row_content(&buf, 0);
        assert!(
            content.contains("md → deg"),
            "mode-change chord label should appear: {:?}",
            content
        );
    }

    // AC-8: mode indicator (left) and settings (right) not displaced by label
    #[test]
    fn test_sides_not_displaced_by_label() {
        let state = CalcState::new(); // defaults: DEG, DEC
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 60, Some("+ → add"));
        let content = row_content(&buf, 0);
        assert!(
            content.starts_with("[NORMAL]"),
            "mode indicator must be at left: {:?}",
            content
        );
        assert!(
            content.contains("DEG  DEC"),
            "settings must appear on right: {:?}",
            content
        );
        // Verify settings are actually rightmost: no non-space chars after the settings block
        let trimmed = content.trim_end();
        assert!(
            trimmed.ends_with("DEG  DEC"),
            "settings should be rightmost content: {:?}",
            content
        );
    }

    // AC-14: mode bar shows SCI notation indicator
    #[test]
    fn test_sci_notation_indicator_shown() {
        use crate::engine::notation::Notation;
        let mut state = CalcState::new();
        state.notation = Notation::Sci;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(content.contains("SCI"), "SCI notation should appear: {:?}", content);
    }

    // AC-14: mode bar shows AUTO notation indicator
    #[test]
    fn test_auto_notation_indicator_shown() {
        use crate::engine::notation::Notation;
        let mut state = CalcState::new();
        state.notation = Notation::Auto;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(content.contains("AUTO"), "AUTO notation should appear: {:?}", content);
    }

    // AC-14: Fixed notation shows no indicator
    #[test]
    fn test_fixed_notation_no_indicator() {
        use crate::engine::notation::Notation;
        let mut state = CalcState::new();
        state.notation = Notation::Fixed;
        let buf = render_mode_bar(&AppMode::Normal, &state, 40);
        let content = row_content(&buf, 0);
        assert!(!content.contains("SCI") && !content.contains("AUTO"),
            "Fixed notation should show no indicator: {:?}", content);
    }

    // [PREC] mode label
    #[test]
    fn test_prec_mode_shows_prec() {
        let state = CalcState::new();
        let buf = render_mode_bar(&AppMode::PrecisionInput(String::new()), &state, 40);
        let content = row_content(&buf, 0);
        assert!(content.contains("[PREC]"), "[PREC] mode should show '[PREC]': {:?}", content);
    }

    // Error condition: label too wide → omitted entirely, no partial display
    #[test]
    fn test_label_omitted_when_too_narrow() {
        let state = CalcState::new();
        // "[NORMAL]" = 8, "RAD  DEC" = 8, "rf → floor" = 10 → need ≥ 32 cols (8+2+10+2+8+2border)
        // Use a width of 20 — too narrow for the label.
        let buf = render_mode_bar_with_cmd(&AppMode::Normal, &state, 20, Some("rf → floor"));
        let content = row_content(&buf, 0);
        assert!(
            !content.contains("rf → floor"),
            "label should be omitted when too narrow, got: {:?}",
            content
        );
        assert!(
            content.contains("[NORMAL]"),
            "mode indicator still present: {:?}",
            content
        );
    }
}
