# Task Manager

A modern, cross-platform terminal-based process manager written in Rust.  
Easily monitor, explore, and interact with system processes in a fast, keyboard and mouse-friendly TUI.

## Features

- **Process Table**
  - View all running processes in a scrollable table
  - Select processes using keyboard (Up/Down arrows) or mouse click

- **Process Details**
  - [x] Display process name
  - [x] List open/listening ports
  - [ ] Show user/owner
  - [ ] Show command line / executable path
  - [ ] Show CPU usage per process
  - [ ] Show RAM usage per process
  - [ ] Show disk usage per process
  - [ ] Show network usage (bytes sent/received)
  - [ ] Show GPU usage per process
  - [ ] Show process start time
  - [ ] Show process status (running, sleeping, etc.)
  - [ ] Show parent/child processes
  - [ ] Show environment variables

- **Resource Overview** *(Planned)*
  - **CPU**
    - [ ] Overview panel
    - [ ] Usage graph
    - [ ] Stats: utilization, speed, process/thread/handle count
    - [ ] Details: base speed, sockets, cores, logical processors, virtualization, caches
  - **RAM**
    - [ ] Overview panel
    - [ ] Usage graph
    - [ ] Stats: in use, available, committed, cached, paged/non-paged pool
    - [ ] Details: speed, slots used, form factor, hardware reserved
  - **GPU**
    - [ ] Overview panel
    - [ ] Usage graphs: 3D, Copy, Video Encode/Decode
    - [ ] Stats: utilization, dedicated/shared memory, temperature
    - [ ] Details: driver version/date, DirectX version, physical location, reserved memory

- **User Interface**
  - [x] Keyboard and mouse navigation
  - [x] Quit with `q` or `Esc`
  - [x] Help bar with controls

*Note: Features marked with [x] are implemented. Others are planned or in progress.*

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/markheramis/task-manager
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
