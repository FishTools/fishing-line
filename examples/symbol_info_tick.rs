use fishing_rod::prelude::*;

fn main() {
    let terminal_path = std::env::var("TERMINAL_PATH").expect("TERMINAL_PATH must be set");

    let runtime = PythonRuntime::new().initialize(&terminal_path);

    if runtime.is_err() {
        panic!("Failed to initialize and connect to MT5 terminal");
    }

    let runtime = runtime.unwrap();

    runtime
        .symbol_select("GBPUSD", Some(true))
        .unwrap_or_else(|_err| {
            panic!("Failed to select GBPUSD");
        });

    let last_tick = runtime
        .symbol_info_tick("GBPUSD")
        .expect("Unable to fetch tick data for `GBPUSD`");

    println!("{:?}", last_tick);

    println!("Show symbol_info_tick(\"GBPUSD\") as struct");

    for (key, value) in last_tick.iter() {
        if let Some(float_value) = value.downcast_ref::<f64>() {
            println!("  {}={}", key, float_value);
        }
        if let Some(integer_value) = value.downcast_ref::<i64>() {
            println!("  {}={}", key, integer_value);
        }
    }
}
