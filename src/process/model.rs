// Process model definition

/// Represents a process and its associated ports.
pub struct Process {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// List of open ports
    pub ports: Vec<u16>,
}