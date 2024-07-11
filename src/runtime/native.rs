use crate::prelude::{MQLError, MQLResult};
use crate::schemas::{
    MQLAccountInfo, MQLCheckResult, MQLHistoryPosition, MQLOrder, MQLPosition, MQLSymbolInfo,
    MQLSymbolRates, MQLSymbolTick, MQLTerminalInfo, MQLTerminalVersion, MQLTradeResult,
};
use crate::traits::{
    AccountInfoTrait, ConnectionTrait, ErrorTrait, HistoryTrait, OrderTrait, PositionTrait,
    SymbolInfoTrait, SymbolRatesTrait, SymbolTicksTrait, TerminalInfoTrait,
};
use chrono::{Datelike, Timelike};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDict};

pub struct NativeRuntime {
    runtime: Option<PyObject>,
}

impl NativeRuntime {
    pub fn new() -> Self {
        let result: PyResult<NativeRuntime> = Python::with_gil(|py| {
            let sys = py
                .import_bound("sys")
                .expect("Unable to import `sys` module");
            let path = sys.getattr("path")?;
            let poetry_environment_path = format!(
                "{}\\lib\\site-packages\\",
                std::env::var("POETRY_ENVIRONMENT").expect("Unable to find `POETRY_ENVIRONMENT`")
            );
            path.getattr("append")?.call1((poetry_environment_path,))?;
            return Ok(NativeRuntime {
                runtime: Some(py.import_bound("MetaTrader5")?.extract()?),
            });
        });
        return result.unwrap();
    }
}

impl ConnectionTrait<NativeRuntime> for NativeRuntime {
    fn initialize_with_credentials(
        self,
        path: &str,
        credentials: crate::schemas::MQLAccountCredentials,
        timeout: i64,
        portable: Option<bool>,
    ) -> MQLResult<NativeRuntime> {
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

impl ErrorTrait for NativeRuntime {
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

impl AccountInfoTrait for NativeRuntime {
    fn account_info(&self) -> crate::prelude::MQLResult<crate::schemas::MQLAccountInfo> {
        let result: PyResult<MQLAccountInfo> = Python::with_gil(|py| {
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

impl TerminalInfoTrait for NativeRuntime {
    fn terminal_info(&self) -> crate::prelude::MQLResult<crate::schemas::MQLTerminalInfo> {
        let result: PyResult<MQLTerminalInfo> = Python::with_gil(|py| {
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
    fn version(&self) -> MQLResult<crate::schemas::MQLTerminalVersion> {
        let result: PyResult<MQLTerminalVersion> = Python::with_gil(|py| {
            let (terminal_version, build, build_date) = self
                .runtime
                .as_ref()
                .unwrap()
                .call_method0(py, "version")?
                .extract::<(i64, i64, String)>(py)?;
            return Ok(MQLTerminalVersion {
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

impl SymbolInfoTrait for NativeRuntime {
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
    fn symbol_info(&self, symbol: &str) -> MQLResult<MQLSymbolInfo> {
        let result: PyResult<MQLSymbolInfo> = Python::with_gil(|py| {
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

    fn symbol_info_tick(&self, symbol: &str) -> MQLResult<MQLSymbolTick> {
        let result: PyResult<MQLSymbolTick> = Python::with_gil(|py| {
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

    fn symbols_get(&self, group: Option<&str>) -> MQLResult<Vec<crate::schemas::MQLSymbolInfo>> {
        let result: PyResult<Vec<MQLSymbolInfo>> = Python::with_gil(|py| {
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

impl SymbolRatesTrait for NativeRuntime {
    fn copy_rates_from(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>> {
        let result: PyResult<Vec<MQLSymbolRates>> = Python::with_gil(|py| {
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
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>> {
        let result: PyResult<Vec<MQLSymbolRates>> = Python::with_gil(|py| {
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
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>> {
        let result: PyResult<Vec<MQLSymbolRates>> = Python::with_gil(|py| {
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

impl SymbolTicksTrait for NativeRuntime {
    fn copy_ticks_from(
        &self,
        symbol: &str,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
        flags: crate::enums::MQLCopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolTick>> {
        let result: PyResult<Vec<MQLSymbolTick>> = Python::with_gil(|py| {
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
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolTick>> {
        let result: PyResult<Vec<MQLSymbolTick>> = Python::with_gil(|py| {
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

impl OrderTrait for NativeRuntime {
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
    fn orders_get(&self) -> MQLResult<Vec<crate::schemas::MQLOrder>> {
        let result: PyResult<Vec<MQLOrder>> = Python::with_gil(|py| {
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
        action: crate::enums::MQLTradeActionRequest,
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
        action: crate::enums::MQLTradeActionRequest,
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
        request: crate::schemas::MQLTradeRequest,
    ) -> MQLResult<crate::schemas::MQLCheckResult> {
        let result: MQLResult<MQLCheckResult> = Python::with_gil(|py| {
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
        request: crate::schemas::MQLTradeRequest,
    ) -> MQLResult<crate::schemas::MQLTradeResult> {
        let result: PyResult<MQLTradeResult> = Python::with_gil(|py| {
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

impl PositionTrait for NativeRuntime {
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
    fn positions_get(&self) -> MQLResult<Vec<crate::schemas::MQLPosition>> {
        let result: PyResult<Vec<MQLPosition>> = Python::with_gil(|py| {
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

impl HistoryTrait for NativeRuntime {
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
    ) -> MQLResult<Vec<MQLOrder>> {
        let result: PyResult<Vec<MQLOrder>> = Python::with_gil(|py| {
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
    ) -> MQLResult<Vec<crate::schemas::MQLHistoryPosition>> {
        let result: PyResult<Vec<MQLHistoryPosition>> = Python::with_gil(|py| {
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
    use crate::{
        schemas::MQLAccountCredentials,
        traits::{ConnectionTrait, TerminalInfoTrait},
    };

    use super::NativeRuntime;

    #[test]
    fn test_connection() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = NativeRuntime::new().initialize(terminal_path.as_str());
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
        let _ = runtime.unwrap().shutdown().unwrap();
    }

    #[test]
    fn test_connection_with_credentials() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let account_credentials = MQLAccountCredentials {
            login: std::env::var("TERMINAL_ACCOUNT_ID")
                .expect("Unable to find `TERMINAL_ACCOUNT_ID` in .env file")
                .parse::<i64>()
                .expect("Unable to parse `TERMINAL_ACCOUNT_ID` to i64"),
            password: std::env::var("TERMINAL_ACCOUNT_PASSWORD")
                .expect("Unable to find `TERMINAL_ACCOUNT_PASSWORD` in .env file"),
            server: std::env::var("TERMINAL_ACCOUNT_SERVER")
                .expect("Unable to find `TERMINAL_ACCOUNT_SERVER` in .env file"),
        };

        let runtime = NativeRuntime::new().initialize_with_credentials(
            terminal_path.as_str(),
            account_credentials,
            1000,
            None,
        );
        assert_eq!(runtime.is_ok(), true, "Unable to connect to terminal");
        let _ = runtime.unwrap().shutdown().unwrap();
    }

    #[test]
    fn test_terminal_version() {
        let terminal_path = std::env::var("TERMINAL_PATH").unwrap();
        let runtime = NativeRuntime::new()
            .initialize(terminal_path.as_str())
            .expect("Unable to connect to terminal");
        let version = runtime.version();
        assert_eq!(version.is_ok(), true, "Unable to get terminal version");
    }
}
