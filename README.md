# Fishing-Rod
Fishing-Rod is a tool that connects the MetaQuotes MetaTrader5 (MT5) platform with Rust programming. It allows you to seamlessly integrate MQL terminals with Rust applications. Fishing-Rod uses Python's native runtime and PyO3 to exchange data between the MQL terminal and your Rust program. This integration enhances the capabilities of MQL platforms and opens up opportunities for financial analysis, algorithmic trading, and data processing within the MQL ecosystem.

## Overview

The MQL platform is widely recognized for its robustness in financial market analysis, algorithmic trading, and custom indicator development. However, integrating these capabilities with the performance and safety of Rust has always been a challenge. Fishing-Rod addresses this gap by providing a Rust implementation that communicates with the MQL terminal via Python's native runtime. This integration is made possible through PyO3, a Rust crate that facilitates the creation of Python extensions in Rust, allowing for direct communication between Rust code and Python scripts.

## How It Works

Fishing-Rod operates by establishing a communication bridge between the MQL terminal and Rust. Here's a simplified explanation of its operation:

1. **Initialization**: The user initializes Fishing-Rod within their Rust application, setting up the necessary configurations to connect with the MQL terminal.

You can initialize connection using terminal path.

```rust
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let runtime = PythonRuntime::new().initialize(terminal_path.as_str());
```

or you can use your account credentials to communicate with the terminal.

```rust
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let account_credentials = AccountCredentials {
    login: std::env::var("TERMINAL_ACCOUNT_ID")
        .expect("Unable to find `TERMINAL_ACCOUNT_ID` in .env file")
        .parse::<i64>()
        .expect("Unable to parse `TERMINAL_ACCOUNT_ID` to i64"),
    password: std::env::var("TERMINAL_ACCOUNT_PASSWORD")
        .expect("Unable to find `TERMINAL_ACCOUNT_PASSWORD` in .env file"),
    server: std::env::var("TERMINAL_ACCOUNT_SERVER")
        .expect("Unable to find `TERMINAL_ACCOUNT_SERVER` in .env file"),
};

let runtime = PythonRuntime::new().initialize_with_credentials(
    terminal_path.as_str(), // terminal path
    account_credentials, // account credentials
    1000, // timeout
    None, // portable mode
);
```


## Installation
currently, this project is under active development. to install the latest version of this project.

```bash
cargo add --git "https://github.com/FishTools/fishing-rod"
```

### Note:
major breaking changes can occur without semantic versioning so be careful and don't use it in production just yet.

To install Metatrader5 and numpy for Fishing-Rod, you need to use poetry as a package manager. Please follow these steps:

1. Create a virtual environment for the Python interpreter by running `poetry shell`.
2. Install the required packages by running `poetry add metatrader5` and `poetry add numpy==1.26.4`. This ensures compatibility with the Metatrader5 package.
3. After installing the virtual environment, get the path where the packages are installed and place it in the .env file using the `POETRY_ENVIRONMENT` variable.

Example .env file:
```env
POETRY_ENVIRONMENT="/path/to/poetry/virtualenv"
```

2. **Data Exchange**: Fishing-Rod uses the Python native runtime and PyO3 to send and receive data between the MQL terminal and Rust.

```rust
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let runtime = PythonRuntime::new()
    .initialize(terminal_path.as_str())
    .expect("Unable to connect to terminal");
let version = runtime.version().expect("Unable to get terminal version");
println!("terminal version: {:?}",version.terminal_version);
println!("terminal build version: {:?}",version.build);
println!("terminal build date: {:?}",version.build_date);
```

3. **Processing**: Once the data is received in Rust, users can leverage Rust's powerful features for data processing, analysis, or algorithmic trading operations.

```rust
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let runtime = PythonRuntime::new()
    .initialize(terminal_path.as_str())
    .expect("Unable to connect to terminal");
// get symbol information
let symbol_info = runtime.symbol_info("EURUSD").expect("Unable to get symbol information");

// analyze the prices starting from now and return 20 bars of price info.
let eurusd_rates = runtime.copy_rates_from("EURUSD",Timeframe::H1,Local::now(),20).expect("Unable to get rates information for this symbol");

// create a trading request
let open_request = TradeRequestBuilder::new()
    .action(enums::TradeActionRequest::DEAL)
    .symbol(symbol_info.name)
    .volume(0.01)
    .price(symbol_info.bid)
    .r#type(enums::OrderType::BUY as i64)
    .type_filling(enums::OrderTypeFilling::IOC)
    .type_time(enums::OrderTypeTime::GTC)
    .sl(symbol_info.bid - (100.0 * symbol_info.point))
    .tp(symbol_info.bid + (100.0 * symbol_info.point))
    .comment("trading from rust!!!".to_string());

// check trade if valid
let check_request = runtime.order_check(open_request);

// check if valid

```

4. **Response**: Processed data or commands can then be sent back to the MQL terminal for further action, such as executing trades.

```rust
let trade_result = runtime.order_send(open_request).expect("Unable to process trade");

if trade_result.retcode == 10009 { // 10009 means `Request completed`
  println!("Order successfully placed");
}
```

## Key Features

- **Seamless Integration**: Smoothly integrates MQL platforms with Rust applications, offering the best of both worlds.
- **Performance**: Leverages Rust's performance and safety features for financial computing and algorithmic trading.
- **Flexibility**: Offers flexibility in data processing and analysis by utilizing Python's extensive libraries and Rust's system-level programming capabilities.
- **Ease of Use**: Simplifies the process of connecting MQL terminals with Rust, making it accessible to both Rust and MQL developers.

Fishing-Rod opens up new possibilities for developers and traders in the financial market, combining the strengths of MQL, Rust, and Python in a unique and powerful way.