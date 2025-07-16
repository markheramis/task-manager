//! TUI application logic: event loop and main rendering

use std::io;
use std::time::{Instant, Duration};
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use tui::backend::CrosstermBackend;
use tui::widgets::TableState;
use tui::Terminal;
use crate::process::get_processes;
use crate::tui::events::handle_events;
use crate::tui::layout::get_chunks;


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
            crate::tui::process::render_process_block(f, &mut table_state, &processes);
            crate::tui::details::render_details_block(f, selected_process);
            crate::tui::help::render_help_bar(f);
        })?;

        match handle_events(&mut table_state, &processes, process_table_area) {
            Ok(0) => (),
            Ok(1) => break,
            Ok(_) => (), // handle all other Ok(u8) cases
            Err(e) => return Err(e),
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
