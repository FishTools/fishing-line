use fishing_rod::prelude::*;

fn main() {
    let terminal_path = std::env::var("TERMINAL_PATH").expect("TERMINAL_PATH must be set");

    let runtime = MT5PythonConnection::new().initialize(&terminal_path);

    if runtime.is_err() {
        panic!("Failed to initialize and connect to MT5 terminal");
    }

    let runtime = runtime.unwrap();

    let symbols = runtime.symbols_total().expect("symbols not found");

    println!("Total symbols: {}", symbols);
}
