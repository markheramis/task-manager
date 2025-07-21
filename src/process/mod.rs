//! Process module: model and fetching logic

pub mod model;
pub mod fetch;

pub use fetch::get_processes; 
pub use model::Process;