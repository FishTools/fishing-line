use std::any::TypeId;

use pyo3;
use pyo3::Python;
use pyo3::{prelude::*, types::PyDict, FromPyObject, IntoPy, PyObject};
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::enums::{self, AccountInfoProperty, RuntimeError, TerminalInfoProperty};
use crate::prelude::{
    AccountMarginMode, AccountStopOutMode, AccountTradeMode, DayOfWeek, DealEntry, DealReason,
    DealType, MQLResult, OrderReason, OrderState, OrderType, OrderTypeFilling, OrderTypeTime,
    PositionReason, PositionType, ReturnCode, SymbolCalcMode, SymbolChartMode,
    SymbolExpirationMode, SymbolFillingMode, SymbolOptionMode, SymbolOptionRight,
    SymbolOrderGtcMode, SymbolOrderMode, SymbolSwapMode, SymbolTradeExecution, SymbolTradeMode,
    TradeActionRequest,
};
use crate::traits::{AccountInfoTrait, InfoProperties, InfoTrait};

#[derive(Serialize, Deserialize, FromPyObject, Debug, Iterable)]
pub struct TerminalVersion {
    pub terminal_version: i64,
    pub build: i64,
    pub build_date: String,
}

#[derive(Serialize, Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct TerminalInfo {
    community_account: bool,
    community_connection: bool,
    connected: bool,
    dlls_allowed: bool,
    trade_allowed: bool,
    email_enabled: bool,
    ftp_enabled: bool,
    notifications_enabled: bool,
    mqid: bool,
    build: i64,
    maxbars: i64,
    codepage: i64,
    ping_last: i64,
    community_balance: f64,
    retransmission: f64,
    company: String,
    name: String,
    language: String,
    path: String,
    data_path: String,
    commondata_path: String,
}

impl InfoTrait for TerminalInfo {
    fn get_info_string(&self, info_property: InfoProperties) -> MQLResult<String> {
        let value = match info_property {
            InfoProperties::TerminalInfoProperty(property) => match property {
                TerminalInfoProperty::Company => self.company.clone(),
                TerminalInfoProperty::Name => self.name.clone(),
                TerminalInfoProperty::Language => self.language.clone(),
                TerminalInfoProperty::Path => self.path.clone(),
                TerminalInfoProperty::DataPath => self.data_path.clone(),
                TerminalInfoProperty::CommonDataPath => self.commondata_path.clone(),
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_integer(&self, info_property: InfoProperties) -> MQLResult<i64> {
        let value = match info_property {
            InfoProperties::TerminalInfoProperty(property) => match property {
                TerminalInfoProperty::Build => self.build,
                TerminalInfoProperty::MaxBars => self.maxbars,
                TerminalInfoProperty::CodePage => self.codepage,
                TerminalInfoProperty::PingLast => self.ping_last,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_float(&self, info_property: InfoProperties) -> MQLResult<f64> {
        let value = match info_property {
            InfoProperties::TerminalInfoProperty(property) => match property {
                TerminalInfoProperty::CommunityBalance => self.community_balance,
                TerminalInfoProperty::Retransmission => self.retransmission,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_boolean(&self, info_property: InfoProperties) -> MQLResult<bool> {
        let value = match info_property {
            InfoProperties::TerminalInfoProperty(property) => match property {
                TerminalInfoProperty::CommunityAccount => self.community_account,
                TerminalInfoProperty::CommunityConnection => self.community_connection,
                TerminalInfoProperty::Connected => self.connected,
                TerminalInfoProperty::DllsAllowed => self.dlls_allowed,
                TerminalInfoProperty::TradeAllowed => self.trade_allowed,
                TerminalInfoProperty::EmailEnabled => self.email_enabled,
                TerminalInfoProperty::FtpEnabled => self.ftp_enabled,
                TerminalInfoProperty::NotificationsEnabled => self.notifications_enabled,
                TerminalInfoProperty::MqId => self.mqid,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
}

#[derive(Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct AccountInfo {
    login: i64,
    trade_mode: AccountTradeMode,
    leverage: i64,
    limit_orders: i64,
    margin_so_mode: AccountStopOutMode,
    trade_allowed: bool,
    trade_expert: bool,
    margin_mode: AccountMarginMode,
    currency_digits: i64,
    fifo_close: bool,
    balance: f64,
    credit: f64,
    profit: f64,
    equity: f64,
    margin: f64,
    margin_free: f64,
    margin_level: f64,
    margin_so_call: f64,
    margin_so_so: f64,
    margin_initial: f64,
    margin_maintenance: f64,
    assets: f64,
    liabilities: f64,
    commission_blocked: f64,
    name: String,
    server: String,
    currency: String,
    company: String,
}

impl InfoTrait for AccountInfo {
    fn get_info_float(&self, info_property: crate::prelude::InfoProperties) -> MQLResult<f64> {
        let value = match info_property {
            InfoProperties::AccountInfoProperty(property) => match property {
                AccountInfoProperty::Balance => self.balance,
                AccountInfoProperty::Profit => self.profit,
                AccountInfoProperty::Equity => self.equity,
                AccountInfoProperty::Margin => self.margin,
                AccountInfoProperty::MarginFree => self.margin_free,
                AccountInfoProperty::MarginLevel => self.margin_level,
                AccountInfoProperty::MarginSoCall => self.margin_so_call,
                AccountInfoProperty::MarginSoSo => self.margin_so_so,
                AccountInfoProperty::MarginInitial => self.margin_initial,
                AccountInfoProperty::MarginMaintenance => self.margin_maintenance,
                AccountInfoProperty::Assets => self.assets,
                AccountInfoProperty::Liabilities => self.liabilities,
                AccountInfoProperty::CommissionBlocked => self.commission_blocked,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_boolean(&self, info_property: InfoProperties) -> MQLResult<bool> {
        let value = match info_property {
            InfoProperties::AccountInfoProperty(property) => match property {
                AccountInfoProperty::TradeAllowed => self.trade_allowed,
                AccountInfoProperty::TradeExpert => self.trade_expert,
                AccountInfoProperty::FifoClose => self.fifo_close,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_integer(&self, info_property: InfoProperties) -> MQLResult<i64> {
        let value = match info_property {
            InfoProperties::AccountInfoProperty(property) => match property {
                AccountInfoProperty::Login => self.login,
                AccountInfoProperty::TradeMode => self.trade_mode as i64,
                AccountInfoProperty::Leverage => self.leverage,
                AccountInfoProperty::LimitOrders => self.limit_orders,
                AccountInfoProperty::MarginSoMode => self.margin_so_mode as i64,
                AccountInfoProperty::MarginMode => self.margin_mode as i64,
                AccountInfoProperty::CurrencyDigits => self.currency_digits,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_string(&self, info_property: InfoProperties) -> MQLResult<String> {
        let value = match info_property {
            InfoProperties::AccountInfoProperty(property) => match property {
                AccountInfoProperty::Currency => self.currency.clone(),
                AccountInfoProperty::Company => self.company.clone(),
                AccountInfoProperty::Name => self.name.clone(),
                AccountInfoProperty::Server => self.server.clone(),
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
}

#[derive(Serialize, FromPyObject)]
pub struct AccountCredentials {
    pub login: i64,
    pub password: String,
    pub server: String,
}

#[derive(Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct SymbolInfo {
    custom: bool,
    chart_mode: SymbolChartMode,
    select: bool,
    visible: bool,
    session_deals: i64,
    session_buy_orders: i64,
    session_sell_orders: i64,
    volume: f64,
    volumehigh: f64,
    volumelow: f64,
    time: i64,
    digits: i64,
    spread: i64,
    spread_float: bool,
    ticks_bookdepth: i64,
    trade_calc_mode: SymbolCalcMode,
    trade_mode: SymbolTradeMode,
    start_time: i64,
    expiration_time: i64,
    trade_stops_level: i64,
    trade_freeze_level: i64,
    trade_exemode: SymbolTradeExecution,
    swap_mode: SymbolSwapMode,
    swap_rollover3days: DayOfWeek,
    margin_hedged_use_leg: bool,
    expiration_mode: SymbolExpirationMode,
    filling_mode: SymbolFillingMode,
    order_mode: SymbolOrderMode,
    order_gtc_mode: SymbolOrderGtcMode,
    option_mode: SymbolOptionMode,
    option_right: SymbolOptionRight,
    bid: f64,
    bidhigh: f64,
    bidlow: f64,
    ask: f64,
    askhigh: f64,
    asklow: f64,
    last: f64,
    lasthigh: f64,
    lastlow: f64,
    volume_real: f64,
    volumehigh_real: f64,
    volumelow_real: f64,
    option_strike: f64,
    point: f64,
    trade_tick_value: f64,
    trade_tick_value_profit: f64,
    trade_tick_value_loss: f64,
    trade_tick_size: f64,
    trade_contract_size: f64,
    trade_accrued_interest: f64,
    trade_face_value: f64,
    trade_liquidity_rate: f64,
    volume_min: f64,
    volume_max: f64,
    volume_step: f64,
    volume_limit: f64,
    swap_long: f64,
    swap_short: f64,
    margin_initial: f64,
    margin_maintenance: f64,
    session_volume: f64,
    session_turnover: f64,
    session_interest: f64,
    session_buy_orders_volume: f64,
    session_sell_orders_volume: f64,
    session_open: f64,
    session_close: f64,
    session_aw: f64,
    session_price_settlement: f64,
    session_price_limit_min: f64,
    session_price_limit_max: f64,
    margin_hedged: f64,
    price_change: f64,
    price_volatility: f64,
    price_theoretical: f64,
    price_greeks_delta: f64,
    price_greeks_theta: f64,
    price_greeks_gamma: f64,
    price_greeks_vega: f64,
    price_greeks_rho: f64,
    price_greeks_omega: f64,
    price_sensitivity: f64,
    basis: String,
    category: String,
    currency_base: String,
    currency_profit: String,
    currency_margin: String,
    bank: String,
    description: String,
    exchange: String,
    formula: String,
    isin: String,
    name: String,
    page: String,
    path: String,
}

impl InfoTrait for SymbolInfo {
    fn get_info_string(&self, info_property: InfoProperties) -> MQLResult<String> {
        let value = match info_property {
            InfoProperties::SymbolInfoProperty(property) => match property {
                enums::SymbolInfoProperty::Basis => self.basis.clone(),
                enums::SymbolInfoProperty::Category => self.category.clone(),
                enums::SymbolInfoProperty::CurrencyBase => self.currency_base.clone(),
                enums::SymbolInfoProperty::CurrencyMargin => self.currency_margin.clone(),
                enums::SymbolInfoProperty::CurrencyProfit => self.currency_profit.clone(),
                enums::SymbolInfoProperty::Bank => self.bank.clone(),
                enums::SymbolInfoProperty::Description => self.description.clone(),
                enums::SymbolInfoProperty::Exchange => self.exchange.clone(),
                enums::SymbolInfoProperty::Formula => self.formula.clone(),
                enums::SymbolInfoProperty::Isin => self.isin.clone(),
                enums::SymbolInfoProperty::Name => self.name.clone(),
                enums::SymbolInfoProperty::Page => self.page.clone(),
                enums::SymbolInfoProperty::Path => self.path.clone(),
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_integer(&self, info_property: InfoProperties) -> MQLResult<i64> {
        let value = match info_property {
            InfoProperties::SymbolInfoProperty(property) => match property {
                enums::SymbolInfoProperty::SessionDeals => self.session_deals,
                enums::SymbolInfoProperty::SessionBuyOrders => self.session_buy_orders,
                enums::SymbolInfoProperty::SessionSellOrders => self.session_sell_orders,
                enums::SymbolInfoProperty::Time => self.time,
                enums::SymbolInfoProperty::Digits => self.digits,
                enums::SymbolInfoProperty::Spread => self.spread,
                enums::SymbolInfoProperty::TicksBookDepth => self.ticks_bookdepth,
                enums::SymbolInfoProperty::StartTime => self.start_time,
                enums::SymbolInfoProperty::ExpirationTime => self.expiration_time,
                enums::SymbolInfoProperty::TradeStopsLevel => self.trade_stops_level,
                enums::SymbolInfoProperty::TradeFreezeLevel => self.trade_freeze_level,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_float(&self, info_property: InfoProperties) -> MQLResult<f64> {
        let value = match info_property {
            InfoProperties::SymbolInfoProperty(property) => match property {
                enums::SymbolInfoProperty::Volume => self.volume,
                enums::SymbolInfoProperty::VolumeHigh => self.volumehigh,
                enums::SymbolInfoProperty::VolumeLow => self.volumelow,
                enums::SymbolInfoProperty::Bid => self.bid,
                enums::SymbolInfoProperty::BidHigh => self.bidhigh,
                enums::SymbolInfoProperty::BidLow => self.bidlow,
                enums::SymbolInfoProperty::Ask => self.ask,
                enums::SymbolInfoProperty::AskHigh => self.askhigh,
                enums::SymbolInfoProperty::AskLow => self.asklow,
                enums::SymbolInfoProperty::Last => self.last,
                enums::SymbolInfoProperty::LastHigh => self.lasthigh,
                enums::SymbolInfoProperty::LastLow => self.lastlow,
                enums::SymbolInfoProperty::VolumeReal => self.volume_real,
                enums::SymbolInfoProperty::VolumeHighReal => self.volumehigh_real,
                enums::SymbolInfoProperty::VolumeLowReal => self.volumelow_real,
                enums::SymbolInfoProperty::OptionStrike => self.option_strike,
                enums::SymbolInfoProperty::Point => self.point,
                enums::SymbolInfoProperty::TradeTickValue => self.trade_tick_value,
                enums::SymbolInfoProperty::TradeTickValueProfit => self.trade_tick_value_profit,
                enums::SymbolInfoProperty::TradeTickValueLoss => self.trade_tick_value_loss,
                enums::SymbolInfoProperty::TradeTickSize => self.trade_tick_size,
                enums::SymbolInfoProperty::TradeContractSize => self.trade_contract_size,
                enums::SymbolInfoProperty::TradeAccruedInterest => self.trade_accrued_interest,
                enums::SymbolInfoProperty::TradeFaceValue => self.trade_face_value,
                enums::SymbolInfoProperty::TradeLiquidityRate => self.trade_liquidity_rate,
                enums::SymbolInfoProperty::VolumeMin => self.volume_min,
                enums::SymbolInfoProperty::VolumeMax => self.volume_max,
                enums::SymbolInfoProperty::VolumeStep => self.volume_step,
                enums::SymbolInfoProperty::VolumeLimit => self.volume_limit,
                enums::SymbolInfoProperty::SwapLong => self.swap_long,
                enums::SymbolInfoProperty::SwapShort => self.swap_short,
                enums::SymbolInfoProperty::MarginInitial => self.margin_initial,
                enums::SymbolInfoProperty::MarginMaintenance => self.margin_maintenance,
                enums::SymbolInfoProperty::SessionVolume => self.session_volume,
                enums::SymbolInfoProperty::SessionTurnover => self.session_turnover,
                enums::SymbolInfoProperty::SessionInterest => self.session_interest,
                enums::SymbolInfoProperty::SessionBuyOrdersVolume => self.session_buy_orders_volume,
                enums::SymbolInfoProperty::SessionSellOrdersVolume => {
                    self.session_sell_orders_volume
                }
                enums::SymbolInfoProperty::SessionOpen => self.session_open,
                enums::SymbolInfoProperty::SessionClose => self.session_close,
                enums::SymbolInfoProperty::SessionAw => self.session_aw,
                enums::SymbolInfoProperty::SessionPriceSettlement => self.session_price_settlement,
                enums::SymbolInfoProperty::SessionPriceLimitMin => self.session_price_limit_min,
                enums::SymbolInfoProperty::SessionPriceLimitMax => self.session_price_limit_max,
                enums::SymbolInfoProperty::MarginHedged => self.margin_hedged,
                enums::SymbolInfoProperty::PriceChange => self.price_change,
                enums::SymbolInfoProperty::PriceVolatility => self.price_volatility,
                enums::SymbolInfoProperty::PriceTheoretical => self.price_theoretical,
                enums::SymbolInfoProperty::PriceGreeksDelta => self.price_greeks_delta,
                enums::SymbolInfoProperty::PriceGreeksTheta => self.price_greeks_theta,
                enums::SymbolInfoProperty::PriceGreeksGamma => self.price_greeks_gamma,
                enums::SymbolInfoProperty::PriceGreeksVega => self.price_greeks_vega,
                enums::SymbolInfoProperty::PriceGreeksRho => self.price_greeks_rho,
                enums::SymbolInfoProperty::PriceGreeksOmega => self.price_greeks_omega,
                enums::SymbolInfoProperty::PriceSensitivity => self.price_sensitivity,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
    fn get_info_boolean(&self, info_property: InfoProperties) -> MQLResult<bool> {
        let value = match info_property {
            InfoProperties::SymbolInfoProperty(property) => match property {
                enums::SymbolInfoProperty::Custom => self.custom,
                enums::SymbolInfoProperty::Select => self.select,
                enums::SymbolInfoProperty::Visible => self.visible,
                enums::SymbolInfoProperty::SpreadFloat => self.spread_float,
                enums::SymbolInfoProperty::MarginHedgedUseLeg => self.margin_hedged_use_leg,
                _ => {
                    panic!("Property not found");
                }
            },
            _ => panic!("Property not found"),
        };
        Ok(value)
    }
}

#[derive(Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct SymbolTick {
    pub time: i64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,
    pub time_msc: i64,
    pub flags: i64,
    pub volume_real: f64,
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct SymbolRates {
    pub time: i64, // change this into date time
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub tick_volume: isize,
    pub spread: f64,
    pub real_volume: isize,
}

#[derive(Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct Order {
    ticket: isize,
    time_setup: i64, // convert this into date time
    #[pyo3(item("type"))]
    r#type: OrderType,
    state: OrderState,
    time_expiration: i64, // convert this into date time
    time_done: i64,       // convert this into date time
    time_setup_msc: isize,
    time_done_msc: isize,
    type_filling: OrderTypeFilling,
    type_time: OrderTypeTime,
    magic: isize,
    reason: OrderReason,
    position_id: isize,
    position_by_id: isize,
    volume_initial: f64,
    volume_current: f64,
    price_open: f64,
    sl: f64,
    tp: f64,
    price_current: f64,
    price_stoplimit: f64,
    symbol: String,
    comment: String,
    external_id: String,
}

#[derive(Deserialize, FromPyObject, Debug, Clone)]
#[pyo3(from_item_all)]
pub struct Position {
    pub ticket: isize,
    pub time: i64, // convert this into date time
    pub time_msc: isize,
    pub time_update: i64, // convert this into date time
    pub time_update_msc: isize,
    #[pyo3(item("type"))]
    pub r#type: PositionType,
    pub magic: isize,
    pub identifier: isize,
    pub reason: PositionReason,
    pub volume: f64,
    pub price_open: f64,
    pub sl: f64,
    pub tp: f64,
    pub price_current: f64,
    pub swap: f64,
    pub profit: f64,
    pub symbol: String,
    pub comment: String,
    pub external_id: String,
}

#[derive(Deserialize, FromPyObject, Debug, Clone)]
#[pyo3(from_item_all)]
pub struct Deals {
    pub ticket: isize,
    pub order: isize,
    pub time: i64, // convert this into date time
    pub time_msc: i64,
    #[pyo3(item("type"))]
    pub r#type: DealType,
    pub entry: DealEntry,
    pub magic: isize,
    pub reason: DealReason,
    pub position_id: isize,
    pub volume: f64,
    pub price: f64,
    pub commission: f64,
    pub swap: f64,
    pub profit: f64,
    pub fee: f64,
    pub symbol: String,
    pub comment: String,
    pub external_id: String,
}

#[derive(Deserialize, FromPyObject, Debug, Clone)]
#[pyo3(from_item_all)]
pub struct TradeRequest {
    pub action: TradeActionRequest,
    pub magic: usize,
    pub order: usize,
    pub symbol: String,
    pub volume: f64,
    pub price: f64,
    pub stoplimit: f64,
    pub sl: f64,
    pub tp: f64,
    pub deviation: usize,
    #[pyo3(item("type"))]
    pub r#type: OrderType,
    pub type_filling: OrderTypeFilling,
    pub type_time: OrderTypeTime,
    pub expiration: i64, // convert this into date time
    pub comment: String,
    pub position: usize,
    pub position_by: usize,
}

#[derive(Default, Clone)]
pub struct TradeRequestBuilder {
    action: Option<TradeActionRequest>,
    magic: Option<i64>,
    order: Option<usize>,
    symbol: Option<String>,
    volume: Option<f64>,
    price: Option<f64>,
    stoplimit: Option<f64>,
    sl: Option<f64>,
    tp: Option<f64>,
    deviation: Option<usize>,
    r#type: Option<OrderType>,
    type_filling: Option<OrderTypeFilling>,
    type_time: Option<OrderTypeTime>,
    expiration: Option<i64>, // convert this into datetime
    comment: Option<String>,
    position: Option<usize>,
    position_by: Option<usize>,
}

impl IntoPy<PyObject> for TradeRequestBuilder {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);
        if self.action.is_some() {
            dict.set_item("action", self.action.unwrap() as i64)
                .unwrap();
        }

        if self.magic.is_some() {
            dict.set_item("magic", self.magic.unwrap()).unwrap();
        }

        if self.order.is_some() {
            dict.set_item("order", self.order.unwrap() as i64).unwrap();
        }

        if self.symbol.is_some() {
            dict.set_item("symbol", self.symbol.unwrap()).unwrap();
        }

        if self.volume.is_some() {
            dict.set_item("volume", self.volume.unwrap()).unwrap();
        }

        if self.price.is_some() {
            dict.set_item("price", self.price.unwrap()).unwrap();
        }

        if self.stoplimit.is_some() {
            dict.set_item("stoplimit", self.stoplimit.unwrap()).unwrap();
        }

        if self.sl.is_some() {
            dict.set_item("sl", self.sl.unwrap()).unwrap();
        }

        if self.tp.is_some() {
            dict.set_item("tp", self.tp.unwrap()).unwrap();
        }

        if self.deviation.is_some() {
            dict.set_item("deviation", self.deviation.unwrap()).unwrap();
        }

        if self.r#type.is_some() {
            dict.set_item("type", self.r#type.unwrap() as i64).unwrap();
        }

        if self.type_filling.is_some() {
            dict.set_item("type_filling", self.type_filling.unwrap() as i64)
                .unwrap();
        }

        if self.type_time.is_some() {
            dict.set_item("type_time", self.type_time.unwrap() as i64)
                .unwrap();
        }

        if self.expiration.is_some() {
            dict.set_item("expiration", self.expiration.unwrap())
                .unwrap();
        }

        if self.comment.is_some() {
            dict.set_item("comment", self.comment.unwrap()).unwrap();
        }

        if self.position.is_some() {
            dict.set_item("position", self.position.unwrap()).unwrap();
        }

        if self.position_by.is_some() {
            dict.set_item("position_by", self.position_by.unwrap())
                .unwrap();
        }

        dict.into_py(py)
    }
}

impl TradeRequestBuilder {
    pub fn new() -> Self {
        TradeRequestBuilder::default()
    }

    pub fn action(mut self, action: enums::TradeActionRequest) -> Self {
        self.action = Some(action);
        self
    }

    pub fn magic(mut self, magic: i64) -> Self {
        self.magic = Some(magic);
        self
    }

    pub fn order(mut self, order: usize) -> Self {
        self.order = Some(order);
        self
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn stoplimit(mut self, stoplimit: f64) -> Self {
        self.stoplimit = Some(stoplimit);
        self
    }

    pub fn sl(mut self, sl: f64) -> Self {
        self.sl = Some(sl);
        self
    }

    pub fn tp(mut self, tp: f64) -> Self {
        self.tp = Some(tp);
        self
    }

    pub fn deviation(mut self, deviation: usize) -> Self {
        self.deviation = Some(deviation);
        self
    }

    pub fn r#type(mut self, r#type: OrderType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn type_filling(mut self, type_filling: enums::OrderTypeFilling) -> Self {
        self.type_filling = Some(type_filling);
        self
    }

    pub fn type_time(mut self, type_time: enums::OrderTypeTime) -> Self {
        self.type_time = Some(type_time);
        self
    }

    pub fn expiration(mut self, expiration: i64) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn position(mut self, position: usize) -> Self {
        self.position = Some(position);
        self
    }

    pub fn position_by(mut self, position_by: usize) -> Self {
        self.position_by = Some(position_by);
        self
    }
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct CheckResult {
    pub retcode: ReturnCode,
    pub balance: f64,
    pub equity: f64,
    pub profit: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
    pub comment: String,
    pub request: TradeRequest,
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct TradeResult {
    pub retcode: ReturnCode,
    pub deal: usize,
    pub order: usize,
    pub volume: f64,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub comment: String,
    pub request_id: u64,
    pub retcode_external: i64,
    pub request: TradeRequest,
}
