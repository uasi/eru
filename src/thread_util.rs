use std::thread;

pub fn spawn_with_name<N, F, T>(name: N, f: F) -> thread::JoinHandle<T>
where
    N: Into<String>,
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name.into()).spawn(f).unwrap()
}
