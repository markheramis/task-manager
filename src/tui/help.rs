//! Help bar rendering for the TUI

use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::tui::layout::get_chunks;

/// Renders the help bar at the bottom of the TUI.
///
/// # Arguments
/// * `f` - The frame to render on
pub fn render_help_bar(f: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    use tui::widgets::{Paragraph, Wrap};
    use tui::text::{Span, Spans};

    let chunks: Vec<Rect> = get_chunks(f);

    // Combine all help into a single line to ensure visibility
    let help_text: Vec<Spans<'_>> = vec![
        Spans::from(vec![
            Span::styled("↑/↓ or Click", Style::default().fg(Color::Yellow)),
            Span::raw(": Selection "),
            Span::styled("q/Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit application"),
        ]),
    ];

    let bottom_block: Block<'_> = Block::default()
        .borders(Borders::ALL)
        .title("Help");
    let help_paragraph: Paragraph<'_> = Paragraph::new(help_text)
        .block(bottom_block)
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, chunks[2]);
} 