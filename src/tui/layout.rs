//! Layout and chunking helpers for the TUI

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};

/// Returns the layout chunks for the main area and the bottom bar.
///
/// # Arguments
/// * `f` - The frame to get the chunks from
///
/// # Returns
/// A vector of Rects: [process_table, details_block, help_bar]
pub fn get_chunks(f: &mut tui::Frame<'_, CrosstermBackend<std::io::Stdout>>) -> Vec<Rect> {
    // Split the screen into a vertical layout with a main area and a bottom bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // main area (will be split horizontally)
            Constraint::Length(3), // bottom bar height
        ].as_ref())
        .split(f.size());

    // Split the main area into two horizontal chunks
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70)
        ].as_ref())
        .split(main_chunks[0]);

    vec![horizontal_chunks[0], horizontal_chunks[1], main_chunks[1]]
} 