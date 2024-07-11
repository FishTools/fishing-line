use pyo3::{
    pyclass,
    types::{PyAnyMethods, PyDict},
    FromPyObject, IntoPy, PyObject, ToPyObject,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, FromPyObject, Debug)]
pub struct MQLTerminalVersion {
    pub terminal_version: i64,
    pub build: i64,
    pub build_date: String,
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLTerminalInfo {
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

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLAccountInfo {
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
pub struct MQLAccountCredentials {
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

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLSymbolInfo {
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

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLSymbolTick {
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
pub struct MQLSymbolRates {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub tick_volume: f64,
    pub spread: f64,
    pub real_volume: f64,
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLOrder {
    pub ticket: i64,
    pub time_setup: i64,
    pub time_setup_msc: i64,
    pub time_expiration: i64,
    #[pyo3(item("type"))]
    pub order_type: i64,
    pub type_time: i64,
    pub type_filling: i64,
    pub state: i64,
    pub magic: i64,
    pub time_done: i64,
    pub time_done_msc: i64,
    pub position_id: i64,
    pub position_by_id: i64,
    pub reason: i64,
    pub volume_initial: f64,
    pub price_stoplimit: f64,
    pub volume_current: f64,
    pub price_open: f64,
    pub sl: f64,
    pub tp: f64,
    pub price_current: f64,
    pub symbol: String,
    pub comment: String,
    pub external_id: String,
}

#[derive(Deserialize, FromPyObject, Debug, Clone)]
#[pyo3(from_item_all)]
pub struct MQLPosition {
    pub ticket: i64,
    pub time: i64,
    pub time_msc: i64,
    pub time_update: i64,
    pub time_update_msc: i64,
    #[pyo3(item("type"))]
    pub r#type: i64,
    pub magic: i64,
    pub identifier: i64,
    pub reason: i64,
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
pub struct MQLHistoryPosition {
    pub ticket: i64,
    pub order: Option<i64>,
    pub time: i64,
    pub time_msc: i64,
    #[pyo3(item("type"))]
    pub r#type: i64,
    pub entry: f64,
    pub magic: i64,
    pub position_id: i64,
    pub reason: i64,
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
pub struct MQLTradeRequest {
    pub action: i64,
    pub magic: i64,
    pub order: Option<i64>,
    pub symbol: String,
    pub volume: f64,
    pub price: f64,
    pub stoplimit: Option<f64>,
    pub sl: f64,
    pub tp: f64,
    pub deviation: Option<i64>,
    #[pyo3(item("type"))]
    pub r#type: i64,
    pub type_filling: i64,
    pub type_time: i64,
    pub expiration: Option<i64>,
    pub comment: String,
    pub position: Option<i64>,
    pub position_by: Option<i64>,
}

impl IntoPy<PyObject> for MQLTradeRequest {
    fn into_py(self, py: pyo3::Python<'_>) -> PyObject {
        let dict = PyDict::new_bound(py);
        dict.set_item("action", self.action).unwrap();
        dict.set_item("magic", self.magic).unwrap();
        if self.order.is_some() {
            dict.set_item("order", self.order.unwrap()).unwrap();
        }
        dict.set_item("symbol", self.symbol).unwrap();
        dict.set_item("volume", self.volume).unwrap();
        dict.set_item("price", self.price).unwrap();
        if self.stoplimit.is_some() {
            dict.set_item("stoplimit", self.stoplimit.unwrap()).unwrap();
        }
        dict.set_item("sl", self.sl).unwrap();
        dict.set_item("tp", self.tp).unwrap();
        if self.deviation.is_some() {
            dict.set_item("deviation", self.deviation.unwrap()).unwrap();
        }
        dict.set_item("type", self.r#type).unwrap();
        dict.set_item("type_filling", self.type_filling).unwrap();
        dict.set_item("type_time", self.type_time).unwrap();
        if self.expiration.is_some() {
            dict.set_item("expiration", self.expiration.unwrap())
                .unwrap();
        }
        if self.position.is_some() {
            dict.set_item("position", self.position.unwrap()).unwrap();
        }
        if self.position_by.is_some() {
            dict.set_item("position_by", self.position_by.unwrap())
                .unwrap();
        }
        dict.set_item("comment", self.comment).unwrap();
        return dict.into_py(py);
    }
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLCheckResult {
    pub retcode: i64,
    pub balance: f64,
    pub equity: f64,
    pub profit: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
    pub comment: String,
    pub request: MQLTradeRequest,
}

#[derive(Deserialize, FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct MQLTradeResult {
    pub retcode: i64,
    pub deal: i64,
    pub order: i64,
    pub volume: f64,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub comment: String,
    pub request_id: i64,
    pub retcode_external: i64,
    pub request: MQLTradeRequest,
}
