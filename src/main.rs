mod process;
mod tui;

// Entry point for the application
fn main() {
    if let Err(e) = tui::run_tui() {
        eprintln!("Error: {}", e);
    }
}