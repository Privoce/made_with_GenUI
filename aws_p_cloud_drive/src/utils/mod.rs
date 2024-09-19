mod aws_structs;
mod commands;
mod state;

pub use aws_structs::*;
pub use commands::*;
pub use state::*;

use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref APP_STATE: Mutex<State> = Mutex::new(State::default());
}
