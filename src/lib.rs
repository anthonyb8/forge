pub mod cli;
mod core;
mod error;
mod utils;

pub use error::{Error, Result};
pub use utils::execute_cmd;
