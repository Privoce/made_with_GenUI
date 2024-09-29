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

pub fn format_size(size: usize) -> String {
    let handle = |n: (u8, f32)| -> (u8, f32) {
        let mut size = n.1;
        let mut level = n.0;
        if n.1 > 1024.0 {
            level += 1;
            size /= 1024.0;
        }

        (level, size)
    };

    let (level, size) = handle((0, size as f32));
    let levels = ["B", "KB", "MB", "GB", "TB"];
    format!("{} {}", size, levels[level as usize])
}
