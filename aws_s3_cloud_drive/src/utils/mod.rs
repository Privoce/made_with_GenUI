mod aws_structs;
mod commands;
mod state;
mod thread;

pub use aws_structs::*;
pub use commands::*;
use lazy_static::lazy_static;
pub use thread::*;

pub use state::*;
use tokio::runtime::Runtime;
use std::{collections::HashMap, sync::Mutex};
lazy_static! {
    pub static ref APP_STATE: Mutex<State> = Mutex::new(State::default());
    pub static ref THREAD_POOL: Runtime = build_thread_pool(4);
    pub static ref LOAD_LIST: Mutex<HashMap<CpId, CpState>> =  Mutex::new(HashMap::new());
}

pub const VIRTUAL_FILE: &str = ".virtual";

pub fn format_size(size: usize) -> String {
    fn handle(n: (u8, f32)) -> (u8, f32) {
        let mut size = n.1;
        let mut level = n.0;

        if size > 1024.0 {
            level += 1;
            size /= 1024.0;
        }

        if size > 1024.0 {
            handle((level, size))
        } else {
            (level, size)
        }
    }

    let (level, size) = handle((0, size as f32));
    let levels = ["B", "KB", "MB", "GB", "TB"];
    format!("{:.2} {}", size, levels[level as usize])
}

#[cfg(test)]
mod t {
    use super::format_size;

    #[test]
    fn format() {
        let s = format_size(120267586);
        dbg!(s);
    }
}
