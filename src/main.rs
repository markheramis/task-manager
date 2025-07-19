mod process;
mod tui;
use log4rs;
// Entry point for the application
fn main() {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    if let Err(e) = tui::run_tui() {
        eprintln!("Error: {}", e);
    }
}