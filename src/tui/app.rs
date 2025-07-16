//! TUI application logic: event loop and main rendering

use std::io;
use std::time::{Instant, Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    widgets::TableState,
    Terminal,
};
use crate::process::get_processes;
use crate::tui::{process_table::render_process_table, layout::get_chunks};

/// Runs the TUI application event loop.
///
/// Handles process selection, refresh, and rendering.
pub fn run_tui() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut table_state = TableState::default();
    table_state.select(Some(0));

    let mut processes = get_processes();
    let mut last_refresh = Instant::now();
    
    loop {
        if last_refresh.elapsed() >= Duration::from_secs(5) {
            processes = get_processes();
            last_refresh = Instant::now();
        }
        let mut process_table_area = None;
        let selected_process = table_state.selected().and_then(|i| processes.get(i));
        terminal.draw(|f| {
            let chunks = get_chunks(f);
            process_table_area = Some(chunks[0]);
            render_process_block(f, &mut table_state, &processes);
            crate::tui::details::render_details_block(f, selected_process);
            crate::tui::help::render_help_bar(f);
        })?;

        // Handle input (quit, up/down, mouse click)
        if event::poll(std::time::Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => {
                        let i = match table_state.selected() {
                            Some(i) => if i == 0 { 0 } else { i - 1 },
                            None => 0,
                        };
                        table_state.select(Some(i));
                    },
                    KeyCode::Down => {
                        let i = match table_state.selected() {
                            Some(i) => if i >= processes.len().saturating_sub(1) { i } else { i + 1 },
                            None => 0,
                        };
                        table_state.select(Some(i));
                    },
                    _ => {}
                },
                Event::Mouse(me) => {
                    if let (Some(area), MouseEventKind::Down(_)) = (process_table_area, me.kind) {
                        if me.column >= area.x && me.column < area.x + area.width && me.row >= area.y + 1 && me.row < area.y + area.height {
                            let row = (me.row - area.y - 1) as usize;
                            if row < processes.len() {
                                table_state.select(Some(row));
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Render the process table block (left panel)
fn render_process_block(
    f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>,
    table_state: &mut TableState,
    processes: &[crate::process::Task],
) {
    let chunks = get_chunks(f);
    let left_block = tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .title("Processes");
    render_process_table(f, left_block, chunks[0], table_state, processes);
}