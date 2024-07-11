use crate::prelude::{MQLError, MQLResult};
use crate::schemas::{
    MQLAccountCredentials, MQLAccountInfo, MQLHistoryPosition, MQLOrder, MQLPosition,
    MQLTerminalInfo, MQLTerminalVersion,
};

pub trait AccountInfoTrait {
    fn account_info(&self) -> MQLResult<MQLAccountInfo>;
}

pub trait TerminalInfoTrait {
    fn terminal_info(&self) -> MQLResult<MQLTerminalInfo>;
    fn version(&self) -> MQLResult<MQLTerminalVersion>;
}

pub trait ConnectionTrait<T> {
    fn initialize(self, path: &str) -> MQLResult<T>;
    fn initialize_with_credentials(
        self,
        path: &str,
        credentials: MQLAccountCredentials,
        timeout: i64,
        portable: Option<bool>,
    ) -> MQLResult<T>;
    fn shutdown(self) -> MQLResult<()>;
}

pub trait ErrorTrait {
    fn last_error(&self) -> MQLError;
}

pub trait SymbolInfoTrait {
    fn symbols_total(&self) -> MQLResult<i32>;
    fn symbols_get(&self, group: Option<&str>) -> MQLResult<Vec<crate::schemas::MQLSymbolInfo>>;
    fn symbol_info(&self, symbol: &str) -> MQLResult<crate::schemas::MQLSymbolInfo>;
    fn symbol_info_tick(&self, symbol: &str) -> MQLResult<crate::schemas::MQLSymbolTick>;
    fn symbol_select(&self, symbol: &str, enable: Option<bool>) -> MQLResult<bool>;
}

pub trait SymbolRatesTrait {
    fn copy_rates_from(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>>;
    fn copy_rates_from_pos(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        start_pos: i32,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>>;
    fn copy_rates_range(
        &self,
        symbol: &str,
        timeframe: crate::enums::MQLTimeframe,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolRates>>;
}

pub trait SymbolTicksTrait {
    fn copy_ticks_from(
        &self,
        symbol: &str,
        date_from: chrono::DateTime<chrono::Local>,
        count: i32,
        flags: crate::enums::MQLCopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolTick>>;
    fn copy_ticks_range(
        &self,
        symbol: &str,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
        flags: crate::enums::MQLCopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::MQLSymbolTick>>;
}

pub trait OrderTrait {
    fn orders_total(&self) -> MQLResult<i64>;
    fn orders_get(&self) -> MQLResult<Vec<crate::schemas::MQLOrder>>;
    fn order_calc_margin(
        &self,
        action: crate::enums::MQLTradeActionRequest,
        symbol: &str,
        volume: f64,
        price: f64,
    ) -> MQLResult<f64>;
    fn order_calc_profit(
        &self,
        action: crate::enums::MQLTradeActionRequest,
        symbol: &str,
        volume: f64,
        price_open: f64,
        price_close: f64,
    ) -> MQLResult<f64>;
    fn order_check(
        &self,
        request: crate::schemas::MQLTradeRequest,
    ) -> MQLResult<crate::schemas::MQLCheckResult>;
    fn order_send(
        &self,
        request: crate::schemas::MQLTradeRequest,
    ) -> MQLResult<crate::schemas::MQLTradeResult>;
}

pub trait PositionTrait {
    fn positions_total(&self) -> MQLResult<i64>;
    fn positions_get(&self) -> MQLResult<Vec<MQLPosition>>;
}

pub trait HistoryTrait {
    fn history_orders_total(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<i64>;
    fn history_orders_get(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<MQLOrder>>;
    fn history_deals_total(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<i64>;
    fn history_deals_get(
        &self,
        date_from: chrono::DateTime<chrono::Local>,
        date_to: chrono::DateTime<chrono::Local>,
    ) -> MQLResult<Vec<MQLHistoryPosition>>;
}
