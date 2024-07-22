use std::any::TypeId;

use crate::enums::{AccountInfoProperty, SymbolInfoProperty, TerminalInfoProperty};
use crate::prelude::{MQLError, MQLResult};
use crate::schemas::{
    AccountCredentials, AccountInfo, Deals, Order, Position, TerminalInfo, TerminalVersion,
};
use chrono::{DateTime, Local};

pub trait AccountInfoTrait {
    fn account_info(&self) -> MQLResult<AccountInfo>;
}

pub trait TerminalInfoTrait {
    fn terminal_info(&self) -> MQLResult<TerminalInfo>;
    fn version(&self) -> MQLResult<TerminalVersion>;
}

pub enum InfoProperties {
    AccountInfoProperty(AccountInfoProperty),
    SymbolInfoProperty(SymbolInfoProperty),
    TerminalInfoProperty(TerminalInfoProperty),
}

pub trait InfoTrait {
    fn get_info_float(&self, info_property: InfoProperties) -> MQLResult<f64>;
    fn get_info_integer(&self, info_property: InfoProperties) -> MQLResult<i64>;
    fn get_info_string(&self, info_property: InfoProperties) -> MQLResult<String>;
    fn get_info_boolean(&self, info_property: InfoProperties) -> MQLResult<bool>;
}

pub trait ConnectionTrait<T> {
    fn initialize(self, path: &str) -> MQLResult<T>;
    fn initialize_with_credentials(
        self,
        path: &str,
        credentials: AccountCredentials,
        timeout: i64,
        portable: Option<bool>,
    ) -> MQLResult<T>;
    fn login(&self, credentials: AccountCredentials, timeout: Option<i64>) -> MQLResult<bool>;
    fn shutdown(self) -> MQLResult<()>;
}
pub trait ErrorTrait {
    fn last_error(&self) -> MQLError;
}

pub trait SymbolInfoTrait {
    fn symbols_total(&self) -> MQLResult<i32>;
    fn symbols_get(&self, group: Option<&str>) -> MQLResult<Vec<crate::schemas::SymbolInfo>>;
    fn symbol_info(&self, symbol: &str) -> MQLResult<crate::schemas::SymbolInfo>;
    fn symbol_info_tick(&self, symbol: &str) -> MQLResult<crate::schemas::SymbolTick>;
    fn symbol_select(&self, symbol: &str, enable: Option<bool>) -> MQLResult<bool>;
}

pub trait SymbolRatesTrait {
    fn copy_rates_from(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        date_from: DateTime<Local>,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>>;
    fn copy_rates_from_pos(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        start_pos: i32,
        count: i32,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>>;
    fn copy_rates_range(
        &self,
        symbol: &str,
        timeframe: crate::enums::Timeframe,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<crate::schemas::SymbolRates>>;
}

pub trait SymbolTicksTrait {
    fn copy_ticks_from(
        &self,
        symbol: &str,
        date_from: DateTime<Local>,
        count: i32,
        flags: crate::enums::CopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>>;
    fn copy_ticks_range(
        &self,
        symbol: &str,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
        flags: crate::enums::CopyTicksFlags,
    ) -> MQLResult<Vec<crate::schemas::SymbolTick>>;
}

pub trait OrderTrait {
    fn orders_total(&self) -> MQLResult<i64>;
    fn orders_get(&self) -> MQLResult<Vec<crate::schemas::Order>>;
    fn order_calc_margin(
        &self,
        action: crate::enums::TradeActionRequest,
        symbol: &str,
        volume: f64,
        price: f64,
    ) -> MQLResult<f64>;
    fn order_calc_profit(
        &self,
        action: crate::enums::TradeActionRequest,
        symbol: &str,
        volume: f64,
        price_open: f64,
        price_close: f64,
    ) -> MQLResult<f64>;
    fn order_check(
        &self,
        request: &crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::CheckResult>;
    fn order_send(
        &self,
        request: crate::schemas::TradeRequestBuilder,
    ) -> MQLResult<crate::schemas::TradeResult>;
}

pub trait PositionTrait {
    fn positions_total(&self) -> MQLResult<i64>;
    fn positions_get(&self) -> MQLResult<Vec<Position>>;
}

pub trait HistoryTrait {
    fn history_orders_total(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<i64>;
    fn history_orders_get(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<Order>>;
    fn history_deals_total(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<i64>;
    fn history_deals_get(
        &self,
        date_from: DateTime<Local>,
        date_to: DateTime<Local>,
    ) -> MQLResult<Vec<Deals>>;
}
