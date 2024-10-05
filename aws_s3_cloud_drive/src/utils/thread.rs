use tokio::runtime::{Builder, Runtime};

pub fn build_thread_pool(num: usize) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(num)
        .enable_all()
        .build()
        .unwrap()
}
