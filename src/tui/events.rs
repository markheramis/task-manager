use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::MouseEventKind;
use tui::layout::Rect;
use tui::widgets::TableState;

pub fn handle_events(
    table_state: &mut TableState,
    processes: &[crate::process::Task],
    process_table_area: Option<Rect>,
) -> Result<u8, std::io::Error> {
    // Handle input (quit, up/down, mouse click)
    if event::poll(std::time::Duration::from_millis(200))? {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(1),
                KeyCode::Up => {
                    let i = match table_state.selected() {
                        Some(i) => if i == 0 { 0 } else { i - 1 },
                        None => 0,
                    };
                    table_state.select(Some(i));
                    return Ok(0);
                },
                KeyCode::Down => {
                    let i = match table_state.selected() {
                        Some(i) => if i >= processes.len().saturating_sub(1) { i } else { i + 1 },
                        None => 0,
                    };
                    table_state.select(Some(i));
                    return Ok(0);
                },
                _ => {
                    return Ok(0);
                }
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
                return Ok(0);
            },
            _ => {
                return Ok(0);
            }
        }
    }
    Ok(0)
}