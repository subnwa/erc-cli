use std::any::Any;

mod manager;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = manager::Manager::new();
}