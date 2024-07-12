use crate::prelude::{MQLError, MQLResult};
use crate::schemas::{
    AccountInfo, CheckResult, HistoryDeals, Order, Position, SymbolInfo, SymbolRates, SymbolTick,
    TerminalInfo, TerminalVersion, TradeResult,
};
use crate::traits::{
    AccountInfoTrait, ConnectionTrait, ErrorTrait, HistoryTrait, OrderTrait, PositionTrait,
    SymbolInfoTrait, SymbolRatesTrait, SymbolTicksTrait, TerminalInfoTrait,
};
use chrono::{Datelike, Timelike};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDict};

pub struct PythonRuntime {
    runtime: Option<PyObject>,
}

impl PythonRuntime {
    pub fn new() -> Self {
        let result: PyResult<PythonRuntime> = Python::with_gil(|py| {
            let sys = py
                .import_bound("sys")
                .expect("Unable to import `sys` module");
            let path = sys.getattr("path")?;
            let poetry_environment_path = format!(
                "{}\\lib\\site-packages\\",
                std::env::var("POETRY_ENVIRONMENT").expect("Unable to find `POETRY_ENVIRONMENT`")
            );
            path.getattr("append")?.call1((poetry_environment_path,))?;
            return Ok(PythonRuntime {
                runtime: Some(py.import_bound("MetaTrader5")?.extract()?),
            });
        });
        return result.unwrap();
    }
}

impl ConnectionTrait<PythonRuntime> for PythonRuntime {
    fn initialize_with_credentials(
        self,
        path: &str,
        credentials: crate::schemas::AccountCredentials,
        timeout: i64,
        portable: Option<bool>,
    ) -> MQLResult<PythonRuntime> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("login", credentials.login).unwrap();
            kwargs.set_item("password", credentials.password).unwrap();
            kwargs.set_item("server", credentials.server).unwrap();
            kwargs.set_item("timeout", timeout).unwrap();
            if portable.is_some() {
                kwargs.set_item("portable", portable.unwrap()).unwrap();
            }
            let runtime = self.runtime.as_ref().unwrap();
            let runtime = runtime
                .getattr(py, "initialize")
                .unwrap()
                .call_bound(py, (path,), Some(&kwargs))
                .unwrap();
            return Ok(runtime.extract(py).unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        if !result.unwrap() {
            return Err((-1, "Failed to initialize MetaTrader5".to_string()));
        }

        return Ok(self);
    }
    fn initialize(self, path: &str) -> MQLResult<Self> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let runtime = self.runtime.as_ref().unwrap();
            let runtime = runtime
                .getattr(py, "initialize")?
                .call1(py, (path,))?
                .extract(py);
            return Ok(runtime.unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        if !result.unwrap() {
            return Err((-1, "Failed to initialize MetaTrader5".to_string()));
        }

        return Ok(self);
    }

    fn shutdown(self) -> MQLResult<()> {
        let _result: PyResult<()> = Python::with_gil(|py| {
            let _runtime = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "shutdown")?
                .call0(py)?;
            return Ok(());
        });

        return Ok(());
    }
}

impl ErrorTrait for PythonRuntime {
    fn last_error(&self) -> crate::prelude::MQLError {
        let result: PyResult<MQLError> = Python::with_gil(|py| {
            let error = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "last_error")?
                .extract(py)?;
            return Ok(error);
        });
        return result.unwrap();
    }
}

impl AccountInfoTrait for PythonRuntime {
    fn account_info(&self) -> crate::prelude::MQLResult<crate::schemas::AccountInfo> {
        let result: PyResult<AccountInfo> = Python::with_gil(|py| {
            let account = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "account_info")?
                .call_method0(py, "_asdict")?
                .extract(py)
                .unwrap();
            return Ok(account);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl TerminalInfoTrait for PythonRuntime {
    fn terminal_info(&self) -> crate::prelude::MQLResult<crate::schemas::TerminalInfo> {
        let result: PyResult<TerminalInfo> = Python::with_gil(|py| {
            let terminal = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "terminal_info")?
                .call_method0(py, "_asdict")?
                .extract(py)
                .unwrap();
            return Ok(terminal);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn version(&self) -> MQLResult<crate::schemas::TerminalVersion> {
        let result: PyResult<TerminalVersion> = Python::with_gil(|py| {
            let (terminal_version, build, build_date) = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "version")?
                .extract::<(i64, i64, String)>(py)?;
            return Ok(TerminalVersion {
                terminal_version,
                build,
                build_date,
            });
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl SymbolInfoTrait for PythonRuntime {
    fn symbols_total(&self) -> MQLResult<i32> {
        let result: PyResult<i32> = Python::with_gil(|py| {
            let total = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "symbols_total")?
                .extract(py)?;
            return Ok(total);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn symbol_info(&self, symbol: &str) -> MQLResult<SymbolInfo> {
        let result: PyResult<SymbolInfo> = Python::with_gil(|py| {
            let symbol = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method1(py, "symbol_info", (symbol,))?
                .getattr(py, "_asdict")?
                .call0(py)?
                .extract(py)?;
            return Ok(symbol);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn symbol_info_tick(&self, symbol: &str) -> MQLResult<SymbolTick> {
        let result: PyResult<SymbolTick> = Python::with_gil(|py| {
            let ticks = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method1(py, "symbol_info_tick", (symbol,))?
                .getattr(py, "_asdict")?
                .call0(py)?
                .extract(py)?;
            return Ok(ticks);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn symbol_select(&self, symbol: &str, enable: Option<bool>) -> crate::prelude::MQLResult<bool> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);

            if enable.is_some() {
                kwargs.set_item("enable", enable).unwrap();
            }

            let selected_symbol = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "symbol_select")?
                .call_bound(py, (symbol,), Some(&kwargs))?
                .extract(py)?;
            return Ok(selected_symbol);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn symbols_get(&self, group: Option<&str>) -> MQLResult<Vec<crate::schemas::SymbolInfo>> {
        let result: PyResult<Vec<SymbolInfo>> = Python::with_gil(|py| {
            let kwargs = PyDict::new_bound(py);
            if group.is_some() {
                kwargs.set_item("group", group.unwrap()).unwrap();
            }
            let symbols = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "symbols_get")?
                .call_bound(py, (), Some(&kwargs))
                .unwrap();
            let symbols_kw = PyDict::new_bound(py);
            symbols_kw.set_item("symbols", symbols).unwrap();
            let symbols = py
                .eval_bound("[s._asdict() for s in symbols]", Some(&symbols_kw), None)?
                .extract()?;
            return Ok(symbols);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl SymbolRatesTrait for PythonRuntime {
    fn copy_rates_from(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "copy_rates_from")?
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
                        .unwrap(),
                        count,
                    ),
                )?;

            let pandas = py.import_bound("pandas").unwrap();

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw.set_item("orient", "records").unwrap();

            let rates = pandas
                .getattr("DataFrame")?
                .call1((rates,))?
                .call_method("to_dict", (), Some(&pandas_kw))?
                .extract()?;

            return Ok(rates);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn copy_rates_from_pos(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        start_pos: i32,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "copy_rates_from_pos")?
                .call1(py, (symbol, timeframe as i64, start_pos, count))?;

            let pandas = py.import_bound("pandas").unwrap();

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw.set_item("orient", "records").unwrap();

            let rates = pandas
                .getattr("DataFrame")?
                .call1((rates,))?
                .call_method("to_dict", (), Some(&pandas_kw))?
                .extract()?;

            return Ok(rates);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn copy_rates_range(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>> {
        let result: PyResult<Vec<SymbolRates>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "copy_rates_range")?
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
                        .unwrap(),
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
                        .unwrap(),
                    ),
                )?;

            let pandas = py.import_bound("pandas").unwrap();

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw.set_item("orient", "records").unwrap();

            let rates = pandas
                .getattr("DataFrame")?
                .call1((rates,))?
                .call_method("to_dict", (), Some(&pandas_kw))?
                .extract()?;

            return Ok(rates);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl SymbolTicksTrait for PythonRuntime {
    fn copy_ticks_from(
        &self,
        symbol: &str,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
        flags: crate::enums::MQLCopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>> {
        let result: PyResult<Vec<SymbolTick>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "copy_ticks_from")?
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
                        .unwrap(),
                        count,
                        flags as i64,
                    ),
                )?;

            let pandas = py.import_bound("pandas").unwrap();

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw.set_item("orient", "records").unwrap();

            let rates = pandas
                .getattr("DataFrame")?
                .call1((rates,))?
                .call_method("to_dict", (), Some(&pandas_kw))?
                .extract()?;

            return Ok(rates);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn copy_ticks_range(
        &self,
        symbol: &str,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
        flags: crate::enums::MQLCopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>> {
        let result: PyResult<Vec<SymbolTick>> = Python::with_gil(|py| {
            let rates = self
                .runtime
                .as_ref()
                .unwrap()
                .getattr(py, "copy_ticks_range")?
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
                        .unwrap(),
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
                        .unwrap(),
                        flags as i64,
                    ),
                )?;

            let pandas = py.import_bound("pandas").unwrap();

            let pandas_kw = PyDict::new_bound(py);

            pandas_kw.set_item("orient", "records").unwrap();

            let rates = pandas
                .getattr("DataFrame")?
                .call1((rates,))?
                .call_method("to_dict", (), Some(&pandas_kw))?
                .extract()?;

            return Ok(rates);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl OrderTrait for PythonRuntime {
    fn orders_total(&self) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_orders = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "orders_total")?
                .extract(py)?;
            return Ok(total_orders);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn orders_get(&self) -> MQLResult<Vec<crate::schemas::Order>> {
        let result: PyResult<Vec<Order>> = Python::with_gil(|py| {
            let orders = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "orders_get")?;

            let orders_kw = PyDict::new_bound(py);

            orders_kw.set_item("orders", orders).unwrap();

            let orders = py
                .eval_bound("[s._asdict() for s in orders]", Some(&orders_kw), None)?
                .extract()?;
            return Ok(orders);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn order_calc_margin(
        &self,
        action: crate::enums::TradeActionRequest,
        symbol: &str,
        volume: f64,
        price: f64,
    ) -> MQLResult<f64> {
        let result: PyResult<f64> = Python::with_gil(|py| {
            let margin = self.runtime.as_ref().unwrap().call_method1(
                py,
                "order_calc_margin",
                (action as i64, symbol, volume, price),
            )?;
            return Ok(margin.extract(py).unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
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
            let profit = self.runtime.as_ref().unwrap().call_method1(
                py,
                "order_calc_profit",
                (action as i64, symbol, volume, price_open, price_close),
            )?;
            return Ok(profit.extract(py).unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }

    fn order_check(
        &self,
        request: crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::CheckResult> {
        let result: MQLResult<CheckResult> = Python::with_gil(|py| {
            let trade_result: PyObject = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method1(py, "order_check", (request,))
                .unwrap()
                .call_method0(py, "_asdict")
                .unwrap();

            let trade_request = trade_result
                .getattr(py, "get")
                .unwrap()
                .call1(py, ("request",))
                .unwrap()
                .call_method0(py, "_asdict")
                .unwrap();

            let update_trade_result_kw = PyDict::new_bound(py);

            update_trade_result_kw
                .set_item("request", trade_request)
                .unwrap();

            trade_result
                .getattr(py, "update")
                .unwrap()
                .call_bound(py, (), Some(&update_trade_result_kw))
                .unwrap();

            let (code, message) = self.last_error();

            if code.is_negative() {
                return Err((code, message));
            }

            return Ok(trade_result.extract(py).unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn order_send(
        &self,
        request: crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::TradeResult> {
        let result: PyResult<TradeResult> = Python::with_gil(|py| {
            let trade_result: PyObject = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method1(py, "order_send", (request,))
                .unwrap()
                .call_method0(py, "_asdict")
                .unwrap();

            let trade_request = trade_result
                .getattr(py, "get")
                .unwrap()
                .call1(py, ("request",))
                .unwrap()
                .call_method0(py, "_asdict")
                .unwrap();

            let update_trade_result_kw = PyDict::new_bound(py);

            update_trade_result_kw
                .set_item("request", trade_request)
                .unwrap();

            trade_result
                .getattr(py, "update")
                .unwrap()
                .call_bound(py, (), Some(&update_trade_result_kw))
                .unwrap();

            return Ok(trade_result.extract(py).unwrap());
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl PositionTrait for PythonRuntime {
    fn positions_total(&self) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_positions = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "positions_total")?
                .extract(py)?;
            return Ok(total_positions);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn positions_get(&self) -> MQLResult<Vec<crate::schemas::Position>> {
        let result: PyResult<Vec<Position>> = Python::with_gil(|py| {
            let positions = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "positions_get")?;

            let positions_kw = PyDict::new_bound(py);

            positions_kw.set_item("positions", positions).unwrap();

            let positions = py
                .eval_bound(
                    "[s._asdict() for s in positions]",
                    Some(&positions_kw),
                    None,
                )?
                .extract()?;
            return Ok(positions);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

impl HistoryTrait for PythonRuntime {
    fn history_orders_total(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_history_orders = self
                .runtime
                .as_ref()
                .unwrap()
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
                        .unwrap(),
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
                        .unwrap(),
                    ),
                )?
                .extract(py)?;
            return Ok(total_history_orders);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn history_orders_get(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<Order>> {
        let result: PyResult<Vec<Order>> = Python::with_gil(|py| {
            let orders = self.runtime.as_ref().unwrap().call_method1(
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
                    .unwrap(),
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
                    .unwrap(),
                ),
            )?;

            let orders_kw = PyDict::new_bound(py);

            orders_kw.set_item("orders", orders).unwrap();

            let orders = py
                .eval_bound("[s._asdict() for s in orders]", Some(&orders_kw), None)?
                .extract()?;
            return Ok(orders);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn history_deals_total(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<i64> {
        let result: PyResult<i64> = Python::with_gil(|py| {
            let total_history_deals = self
                .runtime
                .as_ref()
                .unwrap()
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
                        .unwrap(),
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
                        .unwrap(),
                    ),
                )?
                .extract(py)?;
            return Ok(total_history_deals);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
    fn history_deals_get(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<crate::schemas::HistoryDeals>> {
        let result: PyResult<Vec<HistoryDeals>> = Python::with_gil(|py| {
            let deals = self.runtime.as_ref().unwrap().call_method1(
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
                    .unwrap(),
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
                    .unwrap(),
                ),
            )?;

            let deals_kw = PyDict::new_bound(py);

            deals_kw.set_item("deals", deals).unwrap();

            let deals = py
                .eval_bound("[s._asdict() for s in deals]", Some(&deals_kw), None)?
                .extract()?;

            return Ok(deals);
        });

        let (code, message) = self.last_error();

        if code.is_negative() {
            return Err((code, message));
        }

        return Ok(result.unwrap());
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Local, TimeZone};

    use crate::{
        enums::{self, OrderTypeFilling},
        schemas::{AccountCredentials, TradeRequestBuilder},
        traits::*,
    };

    use super::PythonRuntime;

    #[test]
    fn test_connection() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new().initialize(terminal_path.as_str());
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
    }

    #[test]
    fn test_connection_with_credentials() {
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
            terminal_path.as_str(),
            account_credentials,
            1000,
            None,
        );
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
    }

    #[test]
    fn test_terminal_version() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let version = runtime.version();
        assert_eq!(version.is_ok(), true, "Unable to get terminal version");
    }

    #[test]
    fn test_terminal_info() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let terminal_info = runtime.terminal_info();
        assert_eq!(terminal_info.is_ok(), true, "Unable to get terminal info");
    }

    #[test]
    fn test_account_info() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let account_info = runtime.account_info();
        assert_eq!(account_info.is_ok(), true, "Unable to get account info");
    }

    #[test]
    fn test_symbols_total() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbols_total = runtime.symbols_total();
        assert_eq!(symbols_total.is_ok(), true, "Unable to get symbols total");
    }

    #[test]
    fn test_symbols_get() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_info = runtime.symbol_info("EURUSD");
        assert_eq!(symbol_info.is_ok(), true, "Unable to get symbol info");
    }

    #[test]
    fn test_symbol_info_tick() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_info_tick = runtime.symbol_info_tick("EURUSD");
        assert_eq!(
            symbol_info_tick.is_ok(),
            true,
            "Unable to get symbol info tick"
        );
    }

    #[test]
    fn test_symbol_select() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let symbol_select = runtime.symbol_select("EURUSD", None);
        assert_eq!(symbol_select.is_ok(), true, "Unable to select symbol");
    }

    #[test]
    fn test_copy_rates_from() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_from =
            runtime.copy_rates_from("EURUSD", enums::MQLTimeframe::H1, Local::now(), 20);
        assert_eq!(copy_rates_from.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_rates_from_pos() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_from_pos =
            runtime.copy_rates_from_pos("EURUSD", enums::MQLTimeframe::H1, 0, 20);
        assert_eq!(
            copy_rates_from_pos.is_ok(),
            true,
            "Unable to get symbol rates"
        );
    }

    #[test]
    fn test_copy_rates_range() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_rates_range = runtime.copy_rates_range(
            "EURUSD",
            enums::MQLTimeframe::H1,
            Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap(),
            Local::now(),
        );
        assert_eq!(copy_rates_range.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_ticks_from() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_ticks_from = runtime.copy_ticks_from(
            "EURUSD",
            Local.with_ymd_and_hms(2024, 7, 7, 0, 0, 0).unwrap(),
            20,
            enums::MQLCopyTicksFlags::ALL,
        );
        assert_eq!(copy_ticks_from.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_copy_ticks_range() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let copy_ticks_range = runtime.copy_ticks_range(
            "EURUSD",
            Local.with_ymd_and_hms(2024, 7, 10, 0, 0, 0).unwrap(),
            Local::now(),
            enums::MQLCopyTicksFlags::ALL,
        );
        assert_eq!(copy_ticks_range.is_ok(), true, "Unable to get symbol rates");
    }

    #[test]
    fn test_orders_total() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let orders_total = runtime.orders_total();
        assert_eq!(orders_total.is_ok(), true, "Unable to get total orders");
    }

    #[test]
    fn test_orders_get() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let orders_get = runtime.orders_get();
        assert_eq!(orders_get.is_ok(), true, "Unable to get total orders");
    }

    #[test]
    fn test_order_check() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let current_symbol = runtime.symbol_info("EURUSD").unwrap();

        let check_trade = TradeRequestBuilder::new()
            .action(enums::TradeActionRequest::DEAL)
            .symbol(current_symbol.name)
            .volume(0.01)
            .price(current_symbol.ask)
            .r#type(enums::OrderType::BUY as i64)
            .type_filling(enums::OrderTypeFilling::IOC)
            .type_time(enums::OrderTypeTime::GTC)
            .sl(current_symbol.bid - (100.0 * current_symbol.point))
            .tp(current_symbol.bid + (100.0 * current_symbol.point))
            .comment("Test".to_string());

        let check_order = runtime.order_check(check_trade);

        assert_eq!(check_order.is_ok(), true, "Unable to check order");

        assert_eq!(check_order.unwrap().retcode, 0, "Order is not valid");
    }

    #[test]
    fn test_order_send() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");

        let current_symbol = runtime.symbol_info("EURUSD").unwrap();

        let open_trade = TradeRequestBuilder::new()
            .action(enums::TradeActionRequest::DEAL)
            .symbol(current_symbol.name.clone())
            .volume(0.01)
            .price(current_symbol.ask)
            .r#type(enums::OrderType::BUY as i64)
            .type_filling(enums::OrderTypeFilling::IOC)
            .type_time(enums::OrderTypeTime::GTC)
            .sl(current_symbol.bid - (100.0 * current_symbol.point))
            .tp(current_symbol.bid + (100.0 * current_symbol.point))
            .comment("Test".to_string());

        let open_order = runtime.order_send(open_trade);
        assert_eq!(open_order.is_ok(), true, "Unable to open order");

        let open_order = open_order.unwrap();
        assert_eq!(open_order.retcode, 10009, "Order is not valid");

        let close_request = TradeRequestBuilder::new()
            .action(enums::TradeActionRequest::DEAL)
            .symbol(current_symbol.name)
            .volume(0.01)
            .price(current_symbol.bid)
            .position(open_order.order)
            .r#type(enums::OrderType::SELL as i64)
            .type_filling(OrderTypeFilling::IOC)
            .type_time(enums::OrderTypeTime::GTC)
            .comment("Test".to_string());

        let close_send = runtime.order_send(close_request);

        assert_eq!(close_send.is_ok(), true, "Unable to close order");

        let close_send = close_send.unwrap();

        assert_eq!(close_send.retcode, 10009, "Order is not valid");
    }

    #[test]
    fn test_positions_total() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let positions_get = runtime.positions_get();
        assert_eq!(positions_get.is_ok(), true, "Unable to get total positions");
    }

    #[test]
    fn test_history_orders_total() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = PythonRuntime::new()
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
