//! Details block rendering for the TUI

use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::widgets::{Block, Borders, Paragraph};
use crate::process::Task;

/// Renders the details block showing the selected process's name and ports.
///
/// # Arguments
/// * `f` - The frame to render on
/// * `process` - The selected process to display details for
pub fn render_details_block(
    f: &mut Frame<'_, CrosstermBackend<std::io::Stdout>>,
    process: Option<&Task>,
) {
    use crate::tui::layout::get_chunks;
    let chunks = get_chunks(f);
    let right_block = Block::default()
        .borders(Borders::ALL)
        .title("Details");
    let details = if let Some(proc) = process {
        let ports = if proc.ports.is_empty() {
            "(none)".to_string()
        } else {
            proc.ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
        };
        format!("process: {}\nports: {}", proc.name, ports)
    } else {
        "No process selected".to_string()
    };
    let paragraph = Paragraph::new(details).block(right_block);
    f.render_widget(paragraph, chunks[1]);
} 