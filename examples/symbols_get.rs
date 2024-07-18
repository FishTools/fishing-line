use fishing_rod::prelude::*;

fn main() {
    let terminal_path = std::env::var("TERMINAL_PATH").expect("TERMINAL_PATH must be set");

    let runtime = PythonRuntime::new().initialize(&terminal_path);

    if runtime.is_err() {
        panic!("Failed to initialize and connect to MT5 terminal");
    }

    let runtime = runtime.unwrap();

    let symbols = runtime.symbols_get(None).unwrap();

    println!("Symbols: {}", symbols.len());

    let mut count = 0;
    for symbol in symbols {
        count += 1;
        println!("{}. {}", count, symbol.name);
        if count == 5 {
            break;
        }
    }

    let group_symbols = runtime
        .symbols_get(Some("*,!*USD*,!*EUR*,!*JPY*,!*GBP*"))
        .unwrap();
    println!(
        "len(*,!*USD*,!*EUR*,!*JPY*,!*GBP*): {}",
        group_symbols.len()
    );

    for s in group_symbols {
        println!("{} : {:?}\n", s.name, s);
    }
}
