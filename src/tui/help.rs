//! Help bar rendering for the TUI

use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::widgets::{Block, Borders};

/// Renders the help bar at the bottom of the TUI.
///
/// # Arguments
/// * `f` - The frame to render on
pub fn render_help_bar(f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>) {
    use crate::tui::layout::get_chunks;
    let chunks = get_chunks(f);
    let bottom_block = Block::default()
        .borders(Borders::ALL)
        .title("Help");
    f.render_widget(bottom_block, chunks[2]);
} 