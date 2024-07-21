use pyo3;
use pyo3::Python;
use pyo3::{prelude::*, types::PyDict, FromPyObject, IntoPy, PyObject};
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::enums::{self};
use crate::prelude::{
    DealEntry, DealReason, DealType, OrderType, OrderTypeFilling, OrderTypeTime, PositionReason,
    PositionType, ReturnCode, TradeActionRequest,
};

#[derive(Serialize, Deserialize, FromPyObject, Debug, Iterable)]
pub struct TerminalVersion {
    pub terminal_version: i64,
    pub build: i64,
    pub build_date: String,
}

#[derive(Serialize, Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct TerminalInfo {
    pub community_account: bool,
    pub community_connection: bool,
    pub connected: bool,
    pub dlls_allowed: bool,
    pub trade_allowed: bool,
    pub tradeapi_disabled: bool,
    pub email_enabled: bool,
    pub ftp_enabled: bool,
    pub notifications_enabled: bool,
    pub mqid: bool,
    pub build: i64,
    pub maxbars: i64,
    pub codepage: i64,
    pub ping_last: i64,
    pub community_balance: f64,
    pub retransmission: f64,
    pub company: String,
    pub name: String,
    pub language: String,
    pub path: String,
    pub data_path: String,
    pub commondata_path: String,
}

pub enum TerminalInfoProperty {
    CommunityAccount,
    CommunityConnection,
    Connected,
    DllsAllowed,
    TradeAllowed,
    TradeApiDisabled,
    EmailEnabled,
    FtpEnabled,
    NotificationsEnabled,
    MqId,
    Build,
    MaxBars,
    CodePage,
    PingLast,
    CommunityBalance,
    Retransmission,
    Company,
    Name,
    Language,
    Path,
    DataPath,
    CommonDataPath,
}

#[derive(Serialize, Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct AccountInfo {
    pub login: i64,
    pub trade_mode: i64,
    pub leverage: i64,
    pub limit_orders: i64,
    pub margin_so_mode: i64,
    pub trade_allowed: bool,
    pub trade_expert: bool,
    pub margin_mode: i64,
    pub currency_digits: i64,
    pub fifo_close: bool,
    pub balance: f64,
    pub credit: f64,
    pub profit: f64,
    pub equity: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
    pub margin_so_call: f64,
    pub margin_so_so: f64,
    pub margin_initial: f64,
    pub margin_maintenance: f64,
    pub assets: f64,
    pub liabilities: f64,
    pub commission_blocked: f64,
    pub name: String,
    pub server: String,
    pub currency: String,
    pub company: String,
}

#[derive(Serialize, FromPyObject)]
pub struct AccountCredentials {
    pub login: i64,
    pub password: String,
    pub server: String,
}

pub enum AccountInfoProperty {
    Login,
    TradeMode,
    Leverage,
    LimitOrders,
    MarginSoMode,
    TradeAllowed,
    TradeExpert,
    MarginMode,
    CurrencyDigits,
    FifoClose,
    Balance,
    Credit,
    Profit,
    Equity,
    Margin,
    MarginFree,
    MarginLevel,
    MarginSoCall,
    MarginSoSo,
    MarginInitial,
    MarginMaintenance,
    Assets,
    Liabilities,
    CommissionBlocked,
    Name,
    Server,
    Currency,
    Company,
}

#[derive(Deserialize, FromPyObject, Debug, Iterable)]
#[pyo3(from_item_all)]
pub struct SymbolInfo {
    pub custom: bool,
    pub chart_mode: i64,
    pub select: bool,
    pub visible: bool,
    pub session_deals: i64,
    pub session_buy_orders: i64,
    pub session_sell_orders: i64,
    pub volume: f64,
    pub volumehigh: f64,
    pub volumelow: f64,
    pub time: i64,
    pub digits: i64,
    pub spread: i64,
    pub spread_float: bool,
    pub ticks_bookdepth: i64,
    pub trade_calc_mode: i64,
    pub trade_mode: i64,
    pub start_time: i64,
    pub expiration_time: i64,
    pub trade_stops_level: i64,
    pub trade_freeze_level: i64,
    pub trade_exemode: i64,
    pub swap_mode: i64,
    pub swap_rollover3days: i64,
    pub margin_hedged_use_leg: bool,
    pub expiration_mode: i64,
    pub filling_mode: i64,
    pub order_mode: i64,
    pub order_gtc_mode: i64,
    pub option_mode: i64,
    pub option_right: i64,
    pub bid: f64,
    pub bidhigh: f64,
    pub bidlow: f64,
    pub ask: f64,
    pub askhigh: f64,
    pub asklow: f64,
    pub last: f64,
    pub lasthigh: f64,
    pub lastlow: f64,
    pub volume_real: f64,
    pub volumehigh_real: f64,
    pub volumelow_real: f64,
    pub option_strike: f64,
    pub point: f64,
    pub trade_tick_value: f64,
    pub trade_tick_value_profit: f64,
    pub trade_tick_value_loss: f64,
    pub trade_tick_size: f64,
    pub trade_contract_size: f64,
    pub trade_accrued_interest: f64,
    pub trade_face_value: f64,
    pub trade_liquidity_rate: f64,
    pub volume_min: f64,
    pub volume_max: f64,
    pub volume_step: f64,
    pub volume_limit: f64,
    pub swap_long: f64,
    pub swap_short: f64,
    pub margin_initial: f64,
    pub margin_maintenance: f64,
    pub session_volume: f64,
    pub session_turnover: f64,
    pub session_interest: f64,
    pub session_buy_orders_volume: f64,
    pub session_sell_orders_volume: f64,
    pub session_open: f64,
    pub session_close: f64,
    pub session_aw: f64,
    pub session_price_settlement: f64,
    pub session_price_limit_min: f64,
    pub session_price_limit_max: f64,
    pub margin_hedged: f64,
    pub price_change: f64,
    pub price_volatility: f64,
    pub price_theoretical: f64,
    pub price_greeks_delta: f64,
    pub price_greeks_theta: f64,
    pub price_greeks_gamma: f64,
    pub price_greeks_vega: f64,
    pub price_greeks_rho: f64,
    pub price_greeks_omega: f64,
    pub price_sensitivity: f64,
    pub basis: String,
    pub category: String,
    pub currency_base: String,
    pub currency_profit: String,
    pub currency_margin: String,
    pub bank: String,
    pub description: String,
    pub exchange: String,
    pub formula: String,
    pub isin: String,
    pub name: String,
    pub page: String,
    pub path: String,
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
    /// Order ticket. Unique number assigned to each order
    pub ticket: i64,
    /// Order setup time
    pub time_setup: i64,
    /// The time of placing an order for execution in milliseconds since 01.01.1970
    pub time_setup_msc: i64,
    /// Order expiration time
    pub time_expiration: i64,
    /// Order type
    #[pyo3(item("type"))]
    pub r#type: i64,
    /// Order lifetime
    pub type_time: i64,
    /// Order filling type
    pub type_filling: i64,
    /// Order state
    pub state: i64,
    /// ID of an Expert Advisor that has placed the order (designed to ensure that each Expert Advisor places its own unique number)
    pub magic: i64,
    /// Order execution or cancellation time
    pub time_done: i64,
    /// Order execution/cancellation time in milliseconds since 01.01.1970
    pub time_done_msc: i64,
    /// Position identifier that is set to an order as soon as it is executed. Each executed order results in a deal that opens or modifies an already existing position. The identifier of exactly this position is set to the executed order at this moment.
    pub position_id: i64,
    /// Identifier of an opposite position used for closing by order  ORDER_TYPE_CLOSE_BY
    pub position_by_id: i64,
    /// The reason or source for placing an order
    pub reason: i64,
    /// Order initial volume
    pub volume_initial: f64,
    /// The Limit order price for the StopLimit order
    pub price_stoplimit: f64,
    /// Order current volume
    pub volume_current: f64,
    /// Price specified in the order
    pub price_open: f64,
    /// Stop Loss value
    pub sl: f64,
    /// Take Profit value
    pub tp: f64,
    /// The current price of the order symbol
    pub price_current: f64,
    /// Symbol of the order
    pub symbol: String,
    /// Order comment
    pub comment: String,
    /// Order identifier in an external trading system (on the Exchange)
    pub external_id: String,
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
