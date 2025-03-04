# Fishing-Line
Fishing-Line is a tool that connects the MetaQuotes MetaTrader5 (MT5) platform with Rust programming. It allows you to seamlessly integrate MQL terminals with Rust applications. Fishing-Line uses Python's native runtime and PyO3 to exchange data between the MQL terminal and your Rust program. This integration enhances the capabilities of MQL platforms and opens up opportunities for financial analysis, algorithmic trading, and data processing within the MQL ecosystem.

## Overview

The MQL platform is widely recognized for its robustness in financial market analysis, algorithmic trading, and custom indicator development. However, integrating these capabilities with the performance and safety of Rust has always been a challenge. Fishing-Line addresses this gap by providing a Rust implementation that communicates with the MQL terminal via Python's native runtime. This integration is made possible through PyO3, a Rust crate that facilitates the creation of Python extensions in Rust, allowing for direct communication between Rust code and Python scripts.

## How It Works

Fishing-Line operates by establishing a communication bridge between the MQL terminal and Rust. Here's a simplified explanation of its operation:

1. **Initialization**: The user initializes Fishing-Line within their Rust application, setting up the necessary configurations to connect with the MQL terminal.

You can initialize connection using terminal path.

```rust
use fishing_line::prelude::*;

let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let connection = MT5PythonConnection::new().initialize(&terminal_path);
```

or you can use your account credentials to communicate with the terminal.

```rust
use fishing_line::prelude::*;
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

let connection = MT5PythonConnection::new().initialize_with_credentials(
    &terminal_path, // terminal path
    account_credentials, // account credentials
    1000, // timeout
    None, // portable mode
);
```
## Installation
currently, this project is under active development. to install the latest version of this project.

```bash
cargo add --git "https://github.com/FishTools/Fishing-Line"
```

### Note:
major breaking changes can occur without semantic versioning so be careful and don't use it in production just yet.

To install Metatrader5 and numpy for Fishing-Line, you need to use poetry as a package manager. Please follow these steps:

1. Create a virtual environment for the Python interpreter by running `poetry shell`.
2. Install the required packages by running `poetry add metatrader5` and `poetry add numpy==1.26.4`. This ensures compatibility with the Metatrader5 package.
3. After installing the virtual environment, get the path where the packages are installed and place it in the .env file using the `POETRY_ENVIRONMENT` variable.

Example .env file:
```env
POETRY_ENVIRONMENT="/path/to/poetry/virtualenv"
```

2. **Data Exchange**: Fishing-Line uses the Python native runtime and PyO3 to send and receive data between the MQL terminal and Rust.

```rust
use fishing_line::prelude::*;
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let connection = MT5PythonConnection::new()
    .initialize(terminal_path.as_str())
    .expect("Unable to connect to terminal");
let version = connection.version().expect("Unable to get terminal version");
println!("terminal version: {:?}",version.terminal_version);
println!("terminal build version: {:?}",version.build);
println!("terminal build date: {:?}",version.build_date);
```

3. **Processing**: Once the data is received in Rust, users can leverage Rust's powerful features for data processing, analysis, or algorithmic trading operations and Processed data or commands can then be sent back to the MQL terminal for further action, such as executing trades.

```rust
use fishing_line::prelude::*;
use chrono::Local;
let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
let connection = MT5PythonConnection::new()
    .initialize(terminal_path.as_str())
    .expect("Unable to connect to terminal");

let current_symbol = connection.symbol_info("EURUSD").unwrap();

let open_trade = TradeRequestBuilder::new()
    .action(TradeActionRequest::DEAL)
    .symbol(current_symbol.name.clone())
    .volume(0.01)
    .price(current_symbol.ask)
    .r#type(OrderType::BUY)
    .type_filling(OrderTypeFilling::IOC)
    .type_time(OrderTypeTime::GTC)
    .sl(current_symbol.bid - (100.0 * current_symbol.point))
    .tp(current_symbol.bid + (100.0 * current_symbol.point))
    .comment("Test".to_string());

let open_order = connection.order_send(open_trade).unwrap();

let close_request = TradeRequestBuilder::new()
    .action(TradeActionRequest::DEAL)
    .symbol(current_symbol.name)
    .volume(0.01)
    .price(current_symbol.bid)
    .position(open_order.order)
    .r#type(OrderType::SELL)
    .type_filling(OrderTypeFilling::IOC)
    .type_time(OrderTypeTime::GTC)
    .comment("Test".to_string());

let close_send = connection.order_send(close_request);

let close_send = close_send.unwrap();
```

## Key Features

- **Seamless Integration**: Smoothly integrates MQL platforms with Rust applications, offering the best of both worlds.
- **Performance**: Leverages Rust's performance and safety features for financial computing and algorithmic trading.
- **Flexibility**: Offers flexibility in data processing and analysis by utilizing Python's extensive libraries and Rust's system-level programming capabilities.
- **Ease of Use**: Simplifies the process of connecting MQL terminals with Rust, making it accessible to both Rust and MQL developers.

Fishing-Line opens up new possibilities for developers and traders in the financial market, combining the strengths of MQL, Rust, and Python in a unique and powerful way.
