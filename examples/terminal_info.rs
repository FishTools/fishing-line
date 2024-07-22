use fishing_line::prelude::*;

fn main() {
    let terminal_path = std::env::var("TERMINAL_PATH").expect("TERMINAL_PATH must be set");

    let runtime = MT5PythonConnection::new().initialize(&terminal_path);

    if runtime.is_err() {
        panic!("Failed to initialize and connect to MT5 terminal");
    }

    let runtime = runtime.unwrap();

    let account_credentials = AccountCredentials {
        login: std::env::var("TERMINAL_ACCOUNT_ID")
            .expect("TERMINAL_ACCOUNT_ID must be set")
            .parse::<i64>()
            .unwrap(),
        password: std::env::var("TERMINAL_ACCOUNT_PASSWORD")
            .expect("TERMINAL_PASSWORD must be set"),
        server: std::env::var("TERMINAL_ACCOUNT_SERVER").expect("TERMINAL_SERVER must be set"),
    };

    runtime
        .login(account_credentials, None)
        .unwrap_or_else(|err| {
            let code = err.0;
            let message = err.1;
            panic!("Failed to login: {:?} - {}", code, message);
        });

    let terminal_info = runtime.terminal_info();

    if terminal_info.is_err() {
        panic!("Failed to get account info");
    }

    let terminal_info = terminal_info.unwrap();

    println!("{:?}", terminal_info);

    // use struct_iterable crate to iterate over the account info
    for (key, value) in terminal_info.iter() {
        if let Some(string_value) = value.downcast_ref::<String>() {
            println!("{}: {}", key, string_value);
        }
        if let Some(float_value) = value.downcast_ref::<f64>() {
            println!("{}: {}", key, float_value);
        }
        if let Some(integer_value) = value.downcast_ref::<i64>() {
            println!("{}: {}", key, integer_value);
        }
        if let Some(boolean_value) = value.downcast_ref::<bool>() {
            println!("{}: {}", key, boolean_value);
        }
    }
}
