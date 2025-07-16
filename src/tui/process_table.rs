//! Process table rendering for the TUI

use tui::backend::CrosstermBackend;
use tui::widgets::Table;
use tui::widgets::{Block, Cell, Row};
use tui::layout::Rect;
use tui::Frame;
use tui::style::{Style, Color, Modifier};
use tui::widgets::TableState;
use std::io;
use crate::process::Task;

/// Renders the process table in the given area.
///
/// # Arguments
/// * `f` - The frame to render on
/// * `block` - The block to use for the table
/// * `area` - The area to render the table in
/// * `table_state` - The table state for selection
/// * `processes` - The list of processes to display
pub fn render_process_table(
    f: &mut Frame<'_, CrosstermBackend<io::Stdout>>,
    block: Block,
    area: Rect,
    table_state: &mut TableState,
    processes: &[Task],
) {
    let header = Row::new(vec!["PID", "Name"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    let rows: Vec<Row> = processes.iter().map(|process| {
        Row::new(vec![
            Cell::from(process.pid.to_string()),
            Cell::from(process.name.clone()),
        ])
    }).collect();

    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&[
            tui::layout::Constraint::Length(20),
            tui::layout::Constraint::Length(80),
        ])
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, area, table_state);
}