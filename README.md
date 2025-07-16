# Rust TUI Task Manager

A cross-platform terminal-based process and port viewer built in Rust, using the `tui` and `crossterm` libraries.

## Features
- View running processes in a table
- See open ports for each process
- Select processes with keyboard or mouse
- Responsive TUI layout
- Cross-platform (Windows, Linux, macOS)

## Installation

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd task-manager
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```
3. **Run the application:**
   ```sh
   cargo run --release
   ```

## Usage
- The TUI will launch in your terminal.
- Use the **Up/Down arrow keys** to select a process.
- **Click** on a process row to select it with the mouse.
- The right panel shows the selected process's name and open ports.
- Press **q** or **Esc** to quit.

## Controls
| Key/Action      | Effect                       |
|-----------------|------------------------------|
| Up/Down arrows  | Move selection               |
| Mouse click     | Select process               |
| q or Esc        | Quit the application         |

## Dependencies
- [tui](https://crates.io/crates/tui)
- [crossterm](https://crates.io/crates/crossterm)
- [sysinfo](https://crates.io/crates/sysinfo)
- [netstat2](https://crates.io/crates/netstat2)

## License
MIT 