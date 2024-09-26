mod aws_structs;
mod commands;
mod state;
pub mod threads;

pub use aws_structs::*;
pub use commands::*;
pub use state::*;

use lazy_static::lazy_static;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use threads::build_thread_pool;
use tokio::runtime::Runtime;
lazy_static! {
    pub static ref APP_STATE: Mutex<State> = Mutex::new(State::default());
    pub static ref THREAD_POOL: Runtime = build_thread_pool(4);
    pub static ref TODO_LIST: Mutex<HashMap<CpId, CpState>> = Mutex::new(HashMap::new());
    pub static ref UPLOAD_DIR: Mutex<Option<PathBuf>> = Mutex::new(conf_static());
}

pub const VIRTUAL_FILE: &str = ".virtual";