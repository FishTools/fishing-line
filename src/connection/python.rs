use crate::prelude::*;
use chrono::{DateTime, Local};
use chrono::{Datelike, Timelike};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDict};
use pyo3::PyObject;
use pyo3::Python;

pub struct MT5PythonConnection {
    runtime: Option<PyObject>,
}

impl Default for MT5PythonConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl MT5PythonConnection {
    pub fn new() -> Self {
        let result: PyResult<MT5PythonConnection> = Python::with_gil(|py| {
            let sys = py
                .import_bound("sys")
                .expect("Unable to import `sys` module");
            let path = sys.getattr("path").expect("Unable to get `path` attribute");
            let poetry_environment_path = format!(
                "{}\\lib\\site-packages\\",
                std::env::var("POETRY_ENVIRONMENT").expect("Unable to find `POETRY_ENVIRONMENT`")
            );
            path.getattr("append")
                .expect("Unable to get `append` attribute")
                .call1((poetry_environment_path,))
                .expect("Unable to call `append` method");
            Ok(MT5PythonConnection {
                runtime: Some(
                    py.import_bound("MetaTrader5")
                        .expect("Unable to find `MetaTrader5` module")
                        .extract()
                        .expect("Unable to extract `MetaTrader5` module"),
                ),
            })
        });
        result.expect("Unable to initialize MetaTrader5")
    }
}

impl ConnectionTrait<MT5PythonConnection> for MT5PythonConnection {
    fn login(&self, credentials: AccountCredentials, timeout: Option<i64>) -> MQLResult<bool> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);
            kwargs
                .set_item("password", credentials.password)
                .expect("Unable to set `password`");
            kwargs
                .set_item("server", credentials.server)
                .expect("Unable to set `server`");
            if let Some(timeout) = timeout {
                kwargs
                    .set_item("timeout", timeout)
                    .expect("Unable to set `timeout`");
            }
            let runtime = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module");
            let runtime = runtime
                .getattr(py, "login")
                .expect("Unable to find `login` method")
                .call_bound(py, (credentials.login,), Some(&kwargs))
                .expect("Unable to call `login` method");
            Ok(runtime
                .extract(py)
                .expect("Unable to extract `login` result"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to login"))
    }

    fn initialize_with_credentials(
        self,
        path: &str,
        credentials: crate::schemas::AccountCredentials,
        timeout: i64,
        portable: Option<bool>,
    ) -> MQLResult<MT5PythonConnection> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);
            kwargs
                .set_item("login", credentials.login)
                .expect("Unable to set `login`");
            kwargs
                .set_item("password", credentials.password)
                .expect("Unable to set `password`");
            kwargs
                .set_item("server", credentials.server)
                .expect("Unable to set `server`");
            kwargs
                .set_item("timeout", timeout)
                .expect("Unable to set `timeout`");
            if let Some(portable) = portable {
                kwargs
                    .set_item("portable", portable)
                    .expect("Unable to set `portable`");
            }
            let runtime = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module");
            let runtime = runtime
                .getattr(py, "initialize")
                .expect("Unable to find `initialize` method")
                .call_bound(py, (path,), Some(&kwargs))
                .expect("Unable to call `initialize` method");
            Ok(runtime
                .extract(py)
                .expect("Unable to extract `initialize` result"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        if !result.expect("Unable to initialize MetaTrader5") {
            return Err((
                RuntimeError::AuthFailed,
                "Failed to initialize MetaTrader5".to_string(),
            ));
        }

        Ok(self)
    }
    fn initialize(self, path: &str) -> MQLResult<Self> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let runtime = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module");
            let runtime = runtime
                .getattr(py, "initialize")
                .expect("Unable to find `initialize` method")
                .call1(py, (path,))
                .expect("Unable to call `initialize` method")
                .extract(py);
            Ok(runtime.expect("Unable to extract `initialize` result"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        if !result.expect("Unable to initialize MetaTrader5") {
            return Err((
                RuntimeError::AuthFailed,
                "Failed to initialize MetaTrader5".to_string(),
            ));
        }

        Ok(self)
    }

    fn shutdown(self) -> MQLResult<()> {
        let _result: PyResult<()> = Python::with_gil(|py| {
            let _runtime = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "shutdown")
                .expect("Unable to find `shutdown` method")
                .call0(py)
                .expect("Unable to call `shutdown` method");
            Ok(())
        });

        Ok(())
    }
}

impl ErrorTrait for MT5PythonConnection {
    fn last_error(&self) -> crate::prelude::MQLError {
        let result: PyResult<MQLError> = Python::with_gil(|py| {
            let error = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "last_error")
                .expect("Unable to call `last_error` method")
                .extract(py)
                .expect("Unable to extract `last_error` result");
            Ok(error)
        });
        result.expect("Unable to get last error")
    }
}

impl AccountInfoTrait for MT5PythonConnection {
    fn account_info(&self) -> crate::prelude::MQLResult<crate::schemas::AccountInfo> {
        let result: PyResult<AccountInfo> = Python::with_gil(|py| {
            let account = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "account_info")
                .expect("Unable to call `account_info` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method")
                .extract(py)
                .expect("Unable to extract `account_info` result");
            Ok(account)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get account info"))
    }
}

impl TerminalInfoTrait for MT5PythonConnection {
    fn terminal_info(&self) -> crate::prelude::MQLResult<crate::schemas::TerminalInfo> {
        let result: PyResult<TerminalInfo> = Python::with_gil(|py| {
            let terminal = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "terminal_info")
                .expect("Unable to call `terminal_info` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method")
                .extract(py)
                .expect("Unable to extract `terminal_info` result");
            Ok(terminal)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get terminal info"))
    }
    fn version(&self) -> MQLResult<crate::schemas::TerminalVersion> {
        let result: PyResult<TerminalVersion> = Python::with_gil(|py| {
            let (terminal_version, build, build_date) = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "version")
                .expect("Unable to call `version` method")
                .extract::<(i64, i64, String)>(py)
                .expect("Unable to extract `version` result");
            Ok(TerminalVersion {
                terminal_version,
                build,
                build_date,
            })
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get terminal version"))
    }
}

impl SymbolInfoTrait for MT5PythonConnection {
    fn symbols_total(&self) -> MQLResult<i32> {
        let result: PyResult<i32> = Python::with_gil(|py| {
            let total = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "symbols_total")
                .expect("Unable to call `symbols_total` method")
                .extract(py)
                .expect("Unable to extract `symbols_total` result");
            Ok(total)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get total symbols"))
    }
    fn symbol_info(&self, symbol: &str) -> MQLResult<SymbolInfo> {
        let result: PyResult<SymbolInfo> = Python::with_gil(|py| {
            let symbol = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(py, "symbol_info", (symbol,))
                .expect("Unable to call `symbol_info` method")
                .getattr(py, "_asdict")
                .expect("Unable to get `_asdict` attribute")
                .call0(py)
                .expect("Unable to call `_asdict` method")
                .extract(py)
                .expect("Unable to extract `symbol_info` result");
            Ok(symbol)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get symbol info"))
    }

    fn symbol_info_tick(&self, symbol: &str) -> MQLResult<SymbolTick> {
        let result: PyResult<SymbolTick> = Python::with_gil(|py| {
            let ticks = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(py, "symbol_info_tick", (symbol,))
                .expect("Unable to call `symbol_info_tick` method")
                .getattr(py, "_asdict")
                .expect("Unable to get `_asdict` attribute")
                .call0(py)
                .expect("Unable to call `_asdict` method")
                .extract(py)
                .expect("Unable to extract `symbol_info_tick` result");
            Ok(ticks)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get symbol tick"))
    }

    fn symbol_select(&self, symbol: &str, enable: Option<bool>) -> crate::prelude::MQLResult<bool> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let selected_symbol = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "symbol_select")
                .expect("Unable to find `symbol_select` method")
                .call_bound(py, (symbol, enable.unwrap_or(true)), None)
                .expect("Unable to call `symbol_select` method")
                .extract(py)
                .expect("Unable to extract `symbol_select` result");

            Ok(selected_symbol)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to select symbol"))
    }

    fn symbols_get(&self, group: Option<&str>) -> MQLResult<Vec<crate::schemas::SymbolInfo>> {
        let result: PyResult<Vec<SymbolInfo>> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);
            if let Some(group) = group {
                kwargs
                    .set_item("group", group)
                    .expect("Unable to set `group`");
            }
            let symbols = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "symbols_get")
                .expect("Unable to find `symbols_get` method")
                .call_bound(py, (), Some(&kwargs))
                .expect("Unable to call `symbols_get` method");

            let symbols_kw = PyDict::new_bound(py);
            symbols_kw
                .set_item("symbols", symbols)
                .expect("Unable to set `symbols`");

            let symbols = py
                .eval_bound("[s._asdict() for s in symbols]", Some(&symbols_kw), None)
                .expect("Unable to evaluate `symbols`")
                .extract()
                .expect("Unable to extract `symbols`");
            Ok(symbols)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get symbols"))
    }
}

impl SymbolRatesTrait for MT5PythonConnection {
    fn copy_rates_from(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        date_from: DateTime<Local>,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "copy_rates_from")
                .expect("Unable to find `copy_rates_from` method")
                .call1(
                    py,
                    (
                        symbol,
                        timeframe as i64,
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        count,
                    ),
                )
                .expect("Unable to call `copy_rates_from` method");

            let pandas = py
                .import_bound("pandas")
                .expect("Unable to import `pandas`"); // replace this with polars in the future

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw
                .set_item("orient", "records")
                .expect("Unable to set `orient`");

            let rates = pandas
                .getattr("DataFrame")
                .expect("Unable to get `DataFrame`")
                .call1((rates,))
                .expect("Unable to call `DataFrame`")
                .call_method("to_dict", (), Some(&pandas_kw))
                .expect("Unable to call `to_dict`")
                .extract()
                .expect("Unable to extract `rates`");

            Ok(rates)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to copy rates from"))
    }

    fn copy_rates_from_pos(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        start_pos: i32,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "copy_rates_from_pos")
                .expect("Unable to find `copy_rates_from_pos` method")
                .call1(py, (symbol, timeframe as i64, start_pos, count))
                .expect("Unable to call `copy_rates_from_pos` method");

            let pandas = py
                .import_bound("pandas")
                .expect("Unable to import `pandas`"); // replace this with polars in the future

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw
                .set_item("orient", "records")
                .expect("Unable to set `orient`");

            let rates = pandas
                .getattr("DataFrame")
                .expect("Unable to get `DataFrame`")
                .call1((rates,))
                .expect("Unable to call `DataFrame`")
                .call_method("to_dict", (), Some(&pandas_kw))
                .expect("Unable to call `to_dict`")
                .extract()
                .expect("Unable to extract `rates`");

            Ok(rates)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to copy rates from pos"))
    }

    fn copy_rates_range(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "copy_rates_range")
                .expect("Unable to find `copy_rates_range` method")
                .call1(
                    py,
                    (
                        symbol,
                        timeframe as i64,
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                    ),
                )
                .expect("Unable to call `copy_rates_range` method");

            let pandas = py
                .import_bound("pandas")
                .expect("Unable to import `pandas`"); // replace this with polars in the future

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw
                .set_item("orient", "records")
                .expect("Unable to set `orient`");

            let rates = pandas
                .getattr("DataFrame")
                .expect("Unable to get `DataFrame`")
                .call1((rates,))
                .expect("Unable to call `DataFrame`")
                .call_method("to_dict", (), Some(&pandas_kw))
                .expect("Unable to call `to_dict`")
                .extract()
                .expect("Unable to extract `rates`");

            Ok(rates)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to copy rates range"))
    }
}

impl SymbolTicksTrait for MT5PythonConnection {
    fn copy_ticks_from(
        &self,
        symbol: &str,
        date_from: DateTime<Local>,
        count: i32,
        flags: crate::enums::CopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>> {
        let result: PyResult<Vec<SymbolTick>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "copy_ticks_from")
                .expect("Unable to find `copy_ticks_from` method")
                .call1(
                    py,
                    (
                        symbol,
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        count,
                        flags as i64,
                    ),
                )
                .expect("Unable to call `copy_ticks_from` method");

            let pandas = py
                .import_bound("pandas")
                .expect("Unable to import `pandas`");

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw
                .set_item("orient", "records")
                .expect("Unable to set `orient`");

            let rates = pandas
                .getattr("DataFrame")
                .expect("Unable to get `DataFrame`")
                .call1((rates,))
                .expect("Unable to call `DataFrame`")
                .call_method("to_dict", (), Some(&pandas_kw))
                .expect("Unable to call `to_dict`")
                .extract()
                .expect("Unable to extract `rates`");

            Ok(rates)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to copy ticks from"))
    }
    fn copy_ticks_range(
        &self,
        symbol: &str,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
        flags: crate::enums::CopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>> {
        let result: PyResult<Vec<SymbolTick>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .getattr(py, "copy_ticks_range")
                .expect("Unable to find `copy_ticks_range` method")
                .call1(
                    py,
                    (
                        symbol,
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        flags as i64,
                    ),
                )
                .expect("Unable to call `copy_ticks_range` method");

            let pandas = py
                .import_bound("pandas")
                .expect("Unable to import `pandas`"); // replace this with polars in the future

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw
                .set_item("orient", "records")
                .expect("Unable to set `orient`");

            let rates = pandas
                .getattr("DataFrame")
                .expect("Unable to get `DataFrame`")
                .call1((rates,))
                .expect("Unable to call `DataFrame`")
                .call_method("to_dict", (), Some(&pandas_kw))
                .expect("Unable to call `to_dict`")
                .extract()
                .expect("Unable to extract `rates`");

            Ok(rates)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to copy ticks range"))
    }
}

impl OrderTrait for MT5PythonConnection {
    fn orders_total(&self) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_orders = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "orders_total")
                .expect("Unable to call `orders_total` method")
                .extract(py)
                .expect("Unable to extract `orders_total` result");
            Ok(total_orders)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get total orders"))
    }
    fn orders_get(&self) -> MQLResult<Vec<crate::schemas::Order>> {
        let result: PyResult<Vec<Order>> = Python::with_gil(|py| {
            let orders = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "orders_get")
                .expect("Unable to call `orders_get` method");

            let orders_kw = PyDict::new_bound(py);

            orders_kw
                .set_item("orders", orders)
                .expect("Unable to set `orders`");

            let orders = py
                .eval_bound("[s._asdict() for s in orders]", Some(&orders_kw), None)
                .expect("Unable to evaluate `orders`")
                .extract()
                .expect("Unable to extract `orders`");

            Ok(orders)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get orders"))
    }
    fn order_calc_margin(
        &self,
        action: crate::enums::TradeActionRequest,
        symbol: &str,
        volume: f64,
        price: f64,
    ) -> MQLResult<f64> {
        let result: PyResult<f64> = Python::with_gil(|py| {
            let margin = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "order_calc_margin",
                    (action as i64, symbol, volume, price),
                )
                .expect("Unable to call `order_calc_margin` method");
            Ok(margin.extract(py).expect("Unable to extract `margin`"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to calculate margin"))
    }
    fn order_calc_profit(
        &self,
        action: crate::enums::TradeActionRequest,
        symbol: &str,
        volume: f64,
        price_open: f64,
        price_close: f64,
    ) -> MQLResult<f64> {
        let result: PyResult<f64> = Python::with_gil(|py| {
            let profit = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "order_calc_profit",
                    (action as i64, symbol, volume, price_open, price_close),
                )
                .expect("Unable to call `order_calc_profit` method");
            Ok(profit.extract(py).expect("Unable to extract `profit`"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to calculate profit"))
    }

    fn order_check(
        &self,
        request: &crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::CheckResult> {
        let result: MQLResult<CheckResult> = Python::with_gil(|py| {
            let trade_result: PyObject = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(py, "order_check", (request.clone(),))
                .expect("Unable to call `order_check` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method");

            let trade_request = trade_result
                .getattr(py, "get")
                .expect("Unable to get `get` attribute")
                .call1(py, ("request",))
                .expect("Unable to call `get` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method");

            let update_trade_result_kw = PyDict::new_bound(py);

            update_trade_result_kw
                .set_item("request", trade_request)
                .expect("Unable to set `request`");

            trade_result
                .getattr(py, "update")
                .expect("Unable to get `update` attribute")
                .call_bound(py, (), Some(&update_trade_result_kw))
                .expect("Unable to call `update` method");

            let (code, message) = self.last_error();

            if (code as i64).is_negative() {
                return Err((code, message));
            }

            Ok(trade_result
                .extract(py)
                .expect("Unable to extract `trade_result`"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to check order"))
    }
    fn order_send(
        &self,
        request: crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::TradeResult> {
        let result: PyResult<TradeResult> = Python::with_gil(|py| {
            let trade_result: PyObject = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(py, "order_send", (request,))
                .expect("Unable to call `order_send` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method");

            let trade_request = trade_result
                .getattr(py, "get")
                .expect("Unable to get `get` attribute")
                .call1(py, ("request",))
                .expect("Unable to call `get` method")
                .call_method0(py, "_asdict")
                .expect("Unable to call `_asdict` method");

            let update_trade_result_kw = PyDict::new_bound(py);

            update_trade_result_kw
                .set_item("request", trade_request)
                .expect("Unable to set `request`");

            trade_result
                .getattr(py, "update")
                .expect("Unable to get `update` attribute")
                .call_bound(py, (), Some(&update_trade_result_kw))
                .expect("Unable to call `update` method");

            Ok(trade_result
                .extract(py)
                .expect("Unable to extract `trade_result`"))
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to send order"))
    }
}

impl PositionTrait for MT5PythonConnection {
    fn positions_total(&self) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_positions = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "positions_total")
                .expect("Unable to call `positions_total` method")
                .extract(py)
                .expect("Unable to extract `positions_total` result");
            Ok(total_positions)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get total positions"))
    }
    fn positions_get(&self) -> MQLResult<Vec<crate::schemas::Position>> {
        let result: PyResult<Vec<Position>> = Python::with_gil(|py| {
            let positions = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method0(py, "positions_get")
                .expect("Unable to call `positions_get` method");

            let positions_kw = PyDict::new_bound(py);

            positions_kw
                .set_item("positions", positions)
                .expect("Unable to set `positions`");

            let positions = py
                .eval_bound(
                    "[s._asdict() for s in positions]",
                    Some(&positions_kw),
                    None,
                )
                .expect("Unable to evaluate `positions`")
                .extract()
                .expect("Unable to extract `positions`");
            Ok(positions)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get positions"))
    }
}

impl HistoryTrait for MT5PythonConnection {
    fn history_orders_total(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_history_orders = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "history_orders_total",
                    (
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                    ),
                )
                .expect("Unable to call `history_orders_total` method")
                .extract(py)
                .expect("Unable to extract `total_history_orders`");
            Ok(total_history_orders)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get total history orders"))
    }
    fn history_orders_get(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<Order>> {
        let result: PyResult<Vec<Order>> = Python::with_gil(|py| {
            let orders = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "history_orders_get",
                    (
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                    ),
                )
                .expect("Unable to call `history_orders_get` method");

            let orders_kw = PyDict::new_bound(py);

            orders_kw
                .set_item("orders", orders)
                .expect("Unable to set `orders`");

            let orders = py
                .eval_bound("[s._asdict() for s in orders]", Some(&orders_kw), None)
                .expect("Unable to evaluate `orders`")
                .extract()
                .expect("Unable to extract `orders`");
            Ok(orders)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get history orders"))
    }
    fn history_deals_total(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_history_deals = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "history_deals_total",
                    (
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                    ),
                )
                .expect("Unable to call `history_deals_total` method")
                .extract(py)
                .expect("Unable to extract `total_history_deals");
            Ok(total_history_deals)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get total history deals"))
    }
    fn history_deals_get(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<crate::schemas::Deals>> {
        let result: PyResult<Vec<Deals>> = Python::with_gil(|py| {
            let deals = self
                .runtime
                .as_ref()
                .expect("Unable to find `MetaTrader5` module")
                .call_method1(
                    py,
                    "history_deals_get",
                    (
                        PyDateTime::new_bound(
                            py,
                            date_from.year(),
                            date_from.month() as u8,
                            date_from.day() as u8,
                            date_from.hour() as u8,
                            date_from.minute() as u8,
                            date_from.second() as u8,
                            date_from.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                        PyDateTime::new_bound(
                            py,
                            date_to.year(),
                            date_to.month() as u8,
                            date_to.day() as u8,
                            date_to.hour() as u8,
                            date_to.minute() as u8,
                            date_to.second() as u8,
                            date_to.timestamp_subsec_micros(),
                            None,
                        )
                        .expect("Unable to create `PyDateTime`"),
                    ),
                )
                .expect("Unable to call `history_deals_get` method");

            let deals_kw = PyDict::new_bound(py);

            deals_kw
                .set_item("deals", deals)
                .expect("Unable to set `deals`");

            let deals = py
                .eval_bound("[s._asdict() for s in deals]", Some(&deals_kw), None)
                .expect("Unable to evaluate `deals`")
                .extract()
                .expect("Unable to extract `deals`");

            Ok(deals)
        });

        let (code, message) = self.last_error();

        if (code as i64).is_negative() {
            return Err((code, message));
        }

        Ok(result.expect("Unable to get history deals"))
    }
}

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};

    use crate::prelude::*;

    #[test]
    fn test_connection() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new().initialize(terminal_path.as_str());
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
    }

    #[test]
    fn test_connection_with_credentials() {
        dotenv::dotenv().ok();
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

        let runtime = MT5PythonConnection::new().initialize_with_credentials(
            terminal_path.as_str(),
            account_credentials,
            1000,
            None,
        );
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
    }

    #[test]
    fn test_terminal_version() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let version = runtime.version();
        assert_eq!(version.is_ok(), true, "Unable to get terminal version");
    }

    #[test]
    fn test_terminal_info() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let terminal_info = runtime.terminal_info();
        assert_eq!(terminal_info.is_ok(), true, "Unable to get terminal info");
    }

    #[test]
    fn test_account_info() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let account_info = runtime.account_info();
        assert_eq!(account_info.is_ok(), true, "Unable to get account info");
    }

    #[test]
    fn test_symbols_total() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbols_total = runtime.symbols_total();
        assert_eq!(symbols_total.is_ok(), true, "Unable to get symbols total");
    }

    #[test]
    fn test_symbols_get() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbols_get_all = runtime.symbols_get(None);
        assert_eq!(
            symbols_get_all.is_ok(),
            true,
            "Unable to get all symbols info"
        );
        let symbols_get_specific_group = runtime.symbols_get(Some("*EUR*"));
        assert_eq!(
            symbols_get_specific_group.is_ok(),
            true,
            "Unable to get specific group symbols info"
        );
    }

    #[test]
    fn test_symbol_info() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_info = runtime.symbol_info("BTCUSD");
        assert_eq!(symbol_info.is_ok(), true, "Unable to get symbol info");
    }

    #[test]
    fn test_symbol_info_tick() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_info_tick = runtime.symbol_info_tick("BTCUSD");
        assert_eq!(
            symbol_info_tick.is_ok(),
            true,
            "Unable to get symbol info tick"
        );
    }

    #[test]
    fn test_symbol_select() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_select = runtime.symbol_select("BTCUSD", None);
        assert_eq!(symbol_select.is_ok(), true, "Unable to select symbol");
    }

    #[test]
    fn test_copy_rates_from() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_from = runtime.copy_rates_from("BTCUSD", Timeframe::H1, Local::now(), 20);
        assert_eq!(copy_rates_from.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_rates_from_pos() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_from_pos = runtime.copy_rates_from_pos("BTCUSD", Timeframe::H1, 0, 20);
        assert_eq!(
            copy_rates_from_pos.is_ok(),
            true,
            "Unable to get symbol rates"
        );
    }

    #[test]
    fn test_copy_rates_range() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_range = runtime.copy_rates_range(
            "BTCUSD",
            Timeframe::H1,
            Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap(),
            Local::now(),
        );
        assert_eq!(copy_rates_range.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_ticks_from() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_ticks_from = runtime.copy_ticks_from(
            "BTCUSD",
            Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap(),
            20,
            CopyTicksFlags::ALL,
        );
        assert_eq!(copy_ticks_from.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_ticks_range() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_ticks_range = runtime.copy_ticks_range(
            "BTCUSD",
            Local::now() - chrono::Duration::minutes(20),
            Local::now(),
            CopyTicksFlags::ALL,
        );
        assert_eq!(copy_ticks_range.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_orders_total() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let orders_total = runtime.orders_total();
        assert_eq!(orders_total.is_ok(), true, "Unable to get total orders");
    }

    #[test]
    fn test_orders_get() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let orders_get = runtime.orders_get();
        assert_eq!(orders_get.is_ok(), true, "Unable to get total orders");
    }

    #[test]
    fn test_order_check() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let current_symbol = runtime.symbol_info("BTCUSD").unwrap();

        let symbol_name = current_symbol
            .get_info_string(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Name))
            .unwrap();

        let ask_price = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Ask))
            .unwrap();

        let bid_price = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Bid))
            .unwrap();

        let point = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(
                SymbolInfoProperty::Point,
            ))
            .unwrap();

        let check_trade = TradeRequestBuilder::new()
            .action(TradeActionRequest::DEAL)
            .symbol(symbol_name)
            .volume(0.01)
            .price(ask_price)
            .r#type(OrderType::BUY)
            .type_filling(OrderTypeFilling::IOC)
            .type_time(OrderTypeTime::GTC)
            .sl(bid_price - (100.0 * point))
            .tp(bid_price + (100.0 * point))
            .comment("Test".to_string());

        let check_order = runtime.order_check(&check_trade);

        assert_eq!(check_order.is_ok(), true, "Unable to check order");

        assert_eq!(check_order.unwrap().retcode as u64, 0, "Order is not valid");
    }

    #[test]
    fn test_order_send() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let current_symbol = runtime.symbol_info("BTCUSD").unwrap();

        let symbol_name = current_symbol
            .get_info_string(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Name))
            .unwrap();

        let ask_price = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Ask))
            .unwrap();

        let bid_price = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(SymbolInfoProperty::Bid))
            .unwrap();

        let point = current_symbol
            .get_info_float(InfoProperties::SymbolInfoProperty(
                SymbolInfoProperty::Point,
            ))
            .unwrap();

        let open_trade = TradeRequestBuilder::new()
            .action(TradeActionRequest::DEAL)
            .symbol(symbol_name.clone())
            .volume(0.01)
            .price(ask_price)
            .r#type(OrderType::BUY)
            .type_filling(OrderTypeFilling::IOC)
            .type_time(OrderTypeTime::GTC)
            .sl(bid_price - (100.0 * point))
            .tp(bid_price + (100.0 * point))
            .comment("Test".to_string());

        let open_order = runtime.order_send(open_trade);
        assert_eq!(open_order.is_ok(), true, "Unable to open order");

        let open_order = open_order.unwrap();
        assert_eq!(open_order.retcode, ReturnCode::DONE, "Order is not valid");

        let close_request = TradeRequestBuilder::new()
            .action(TradeActionRequest::DEAL)
            .symbol(symbol_name)
            .volume(0.01)
            .price(bid_price)
            .position(open_order.order)
            .r#type(OrderType::SELL)
            .type_filling(OrderTypeFilling::IOC)
            .type_time(OrderTypeTime::GTC)
            .comment("Test".to_string());

        let close_send = runtime.order_send(close_request);

        assert_eq!(close_send.is_ok(), true, "Unable to close order");

        let close_send = close_send.unwrap();

        assert_eq!(close_send.retcode, ReturnCode::DONE, "Order is not valid");
    }

    #[test]
    fn test_positions_total() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let positions_total = runtime.positions_total();
        assert_eq!(
            positions_total.is_ok(),
            true,
            "Unable to get total positions"
        );
    }

    #[test]
    fn test_positions_get() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let positions_get = runtime.positions_get();
        assert_eq!(positions_get.is_ok(), true, "Unable to get total positions");
    }

    #[test]
    fn test_history_orders_total() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let date_from = Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap();
        let date_to = Local::now();

        let history_orders_total = runtime.history_orders_total(date_from, date_to);
        assert_eq!(
            history_orders_total.is_ok(),
            true,
            "Unable to get total history orders"
        );
    }

    #[test]
    fn test_history_orders_get() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let date_from = Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap();
        let date_to = Local::now();

        let history_orders_get = runtime.history_orders_get(date_from, date_to);
        assert_eq!(
            history_orders_get.is_ok(),
            true,
            "Unable to get history orders"
        );
    }

    #[test]
    fn test_history_deals_total() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let date_from = Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap();
        let date_to = Local::now();

        let history_deals_total = runtime.history_deals_total(date_from, date_to);
        assert_eq!(
            history_deals_total.is_ok(),
            true,
            "Unable to get total history deals"
        );
    }

    #[test]
    fn test_history_deals_get() {
        dotenv::dotenv().ok();
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = MT5PythonConnection::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let date_from = Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap();
        let date_to = Local::now();

        let history_deals_get = runtime.history_deals_get(date_from, date_to);
        assert_eq!(
            history_deals_get.is_ok(),
            true,
            "Unable to get history deals"
        );
    }
}
