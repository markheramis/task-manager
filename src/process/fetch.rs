//! Process and port fetching logic

use sysinfo::{System};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use super::model::Task;

/// Fetches all processes and their open ports.
pub fn get_processes() -> Vec<Task> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut tasks = Vec::new();
    for (pid, process) in system.processes() {
        let pid_u32: u32 = pid.as_u32();
        let ports: Vec<u16> = get_ports_by_pid(pid_u32);
        
        tasks.push(Task {
            pid: pid_u32,
            name: process.name().to_string_lossy().to_string(),
            ports: ports
        });
    }
    tasks
}

/// Returns a list of open ports for a given process ID.
fn get_ports_by_pid(pid: u32) -> Vec<u16> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let mut ports = Vec::new();

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        for si in sockets {
            if let Some(socket_pid) = si.associated_pids.first() {
                if *socket_pid == pid {
                    let port = match si.protocol_socket_info {
                        ProtocolSocketInfo::Tcp(tcp) => tcp.local_port,
                        ProtocolSocketInfo::Udp(udp) => udp.local_port,
                    };
                    ports.push(port);
                }
            }
        }
    }
    ports
} 