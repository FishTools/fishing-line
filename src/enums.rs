use pyo3::{types::PyAnyMethods, FromPyObject};
use serde::Deserialize;

use crate::traits::InfoTrait;

/// Represents the timeframe for a trading operation.
#[derive(Debug, Clone, Copy)]
pub enum Timeframe {
    /// 1 minute
    M1 = 1,
    /// 2 minutes
    M2 = 2,
    /// 3 minutes
    M3 = 3,
    /// 4 minutes
    M4 = 4,
    /// 5 minutes
    M5 = 5,
    /// 6 minutes
    M6 = 6,
    /// 10 minutes
    M10 = 10,
    /// 12 minutes
    M12 = 12,
    /// 15 minutes
    M15 = 15,
    /// 20 minutes
    M20 = 20,
    /// 30 minutes
    M30 = 30,
    /// 1 hour
    H1 = 1 | 0x4000,
    /// 2 hours
    H2 = 2 | 0x4000,
    /// 3 hours
    H3 = 3 | 0x4000,
    /// 4 hours
    H4 = 4 | 0x4000,
    /// 6 hours
    H6 = 6 | 0x4000,
    /// 8 hours
    H8 = 8 | 0x4000,
    /// 12 hours
    H12 = 12 | 0x4000,
    /// 1 day
    D1 = 24 | 0x4000,
    /// 1 week
    W1 = 1 | 0x8000,
    /// 1 month
    MN1 = 1 | 0xC000,
}

/// Represents the flags for copying ticks.
#[derive(Debug, Clone, Copy)]
pub enum CopyTicksFlags {
    ALL = -1,
    INFO = 1,
    TRADE = 2,
}

/// Represents the flags for ticks.
#[derive(Debug, Clone, Copy)]
pub enum TicksFlag {
    BID = 0x02,
    ASK = 0x04,
    LAST = 0x08,
    VOLUME = 0x10,
    BUY = 0x20,
    SELL = 0x40,
}

/// Represents the type of an order.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum OrderType {
    BUY = 0,
    SELL = 1,
    BuyLimit = 2,
    SellLimit = 3,
    BuyStop = 4,
    SellStop = 5,
    BuyStopLimit = 6,
    SellStopLimit = 7,
    CloseBy = 8,
}

impl FromPyObject<'_> for OrderType {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();
        match value {
            0 => Ok(OrderType::BUY),
            1 => Ok(OrderType::SELL),
            2 => Ok(OrderType::BuyLimit),
            3 => Ok(OrderType::SellLimit),
            4 => Ok(OrderType::BuyStop),
            5 => Ok(OrderType::SellStop),
            6 => Ok(OrderType::BuyStopLimit),
            7 => Ok(OrderType::SellStopLimit),
            8 => Ok(OrderType::CloseBy),
            _ => {
                panic!("Invalid OrderType value: {}", value)
            }
        }
    }
}

impl From<i64> for OrderType {
    /// Converts an `i64` value to an `OrderType`.
    fn from(value: i64) -> Self {
        match value {
            0 => OrderType::BUY,
            1 => OrderType::SELL,
            2 => OrderType::BuyLimit,
            3 => OrderType::SellLimit,
            4 => OrderType::BuyStop,
            5 => OrderType::SellStop,
            6 => OrderType::BuyStopLimit,
            7 => OrderType::SellStopLimit,
            8 => OrderType::CloseBy,
            _ => panic!("Invalid OrderType value: {}", value),
        }
    }
}

/// Represents a trade action request.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TradeActionRequest {
    DEAL = 1,
    PENDING = 5,
    SLTP = 6,
    MODIFY = 7,
    REMOVE = 8,
    CloseBy = 10,
}

impl FromPyObject<'_> for TradeActionRequest {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();
        match value {
            1 => Ok(TradeActionRequest::DEAL),
            5 => Ok(TradeActionRequest::PENDING),
            6 => Ok(TradeActionRequest::SLTP),
            7 => Ok(TradeActionRequest::MODIFY),
            8 => Ok(TradeActionRequest::REMOVE),
            10 => Ok(TradeActionRequest::CloseBy),
            _ => {
                panic!("Invalid TradeActionRequest value");
            }
        }
    }
}

impl From<i64> for TradeActionRequest {
    /// Converts an `i64` value to a `TradeActionRequest`.
    fn from(value: i64) -> Self {
        match value {
            1 => TradeActionRequest::DEAL,
            5 => TradeActionRequest::PENDING,
            6 => TradeActionRequest::SLTP,
            7 => TradeActionRequest::MODIFY,
            8 => TradeActionRequest::REMOVE,
            10 => TradeActionRequest::CloseBy,
            _ => panic!("Invalid TradeActionRequest value: {}", value),
        }
    }
}

/// Represents the filling type for an order.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum OrderTypeFilling {
    FOK = 0,
    IOC = 1,
    RETURN = 2,
}

impl From<i64> for OrderTypeFilling {
    /// Converts an `i64` value to an `OrderTypeFilling`.
    fn from(value: i64) -> Self {
        match value {
            0 => OrderTypeFilling::FOK,
            1 => OrderTypeFilling::IOC,
            2 => OrderTypeFilling::RETURN,
            _ => panic!("Invalid OrderTypeFilling value: {}", value),
        }
    }
}

impl FromPyObject<'_> for OrderTypeFilling {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();

        match value {
            0 => Ok(OrderTypeFilling::FOK),
            1 => Ok(OrderTypeFilling::IOC),
            2 => Ok(OrderTypeFilling::RETURN),
            _ => panic!("Invalid OrderTypeFilling value: {}", value),
        }
    }
}

/// Represents the time type for an order.
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum OrderTypeTime {
    GTC = 0,
    DAY = 1,
    SPECIFIED = 2,
    SpecifiedDay = 3,
}

impl From<i64> for OrderTypeTime {
    /// Converts an `i64` value to an `OrderTypeTime`.
    fn from(value: i64) -> Self {
        match value {
            0 => OrderTypeTime::GTC,
            1 => OrderTypeTime::DAY,
            2 => OrderTypeTime::SPECIFIED,
            3 => OrderTypeTime::SpecifiedDay,
            _ => panic!("Invalid OrderTypeTime value: {}", value),
        }
    }
}

impl FromPyObject<'_> for OrderTypeTime {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();

        match value {
            0 => Ok(OrderTypeTime::GTC),
            1 => Ok(OrderTypeTime::DAY),
            2 => Ok(OrderTypeTime::SPECIFIED),
            3 => Ok(OrderTypeTime::SpecifiedDay),
            _ => panic!("Invalid OrderTypeTime value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum ReturnCode {
    CHECKED = 0,
    REQUOTE = 10004,
    REJECT = 10006,
    CANCELLED = 10007,
    PLACED = 10008,
    DONE = 10009,
    DonePartial = 10010,
    ERROR = 10011,
    TIMEOUT = 10012,
    INVALID = 10013,
    InvalidVolume = 10014,
    InvalidPrice = 10015,
    InvalidStops = 10016,
    TradeDisabled = 10017,
    MarketClosed = 10018,
    NoMoney = 10019,
    PriceChanged = 10020,
    PriceOff = 10021,
    InvalidExpiration = 10022,
    OrderChanged = 10023,
    TooManyRequest = 10024,
    NoChanges = 10025,
    ServerDisablesAt = 10026,
    ClientDisablesAt = 10027,
    LOCKED = 10028,
    FROZEN = 10029,
    InvalidFill = 10030,
    CONNECTION = 10031,
    OnlyReal = 10032,
    LimitOrders = 10033,
    LimitVolumes = 10034,
    InvalidOrders = 10035,
    PostionClosed = 10036,
    InvalideCloseVolume = 10038,
    CloseOrderExist = 10039,
    LimitPositions = 10040,
    RejectCancel = 10041,
    LongOnly = 10042,
    ShortOnly = 10043,
    CloseOnly = 10044,
    FifoClose = 10045,
    HedgeProhibited = 10046,
}

impl From<u64> for ReturnCode {
    fn from(value: u64) -> Self {
        match value {
            0 => ReturnCode::CHECKED,
            10004 => ReturnCode::REQUOTE,
            10006 => ReturnCode::REJECT,
            10007 => ReturnCode::CANCELLED,
            10008 => ReturnCode::PLACED,
            10009 => ReturnCode::DONE,
            10010 => ReturnCode::DonePartial,
            10011 => ReturnCode::ERROR,
            10012 => ReturnCode::TIMEOUT,
            10013 => ReturnCode::INVALID,
            10014 => ReturnCode::InvalidVolume,
            10015 => ReturnCode::InvalidPrice,
            10016 => ReturnCode::InvalidStops,
            10017 => ReturnCode::TradeDisabled,
            10018 => ReturnCode::MarketClosed,
            10019 => ReturnCode::NoMoney,
            10020 => ReturnCode::PriceChanged,
            10021 => ReturnCode::PriceOff,
            10022 => ReturnCode::InvalidExpiration,
            10023 => ReturnCode::OrderChanged,
            10024 => ReturnCode::TooManyRequest,
            10025 => ReturnCode::NoChanges,
            10026 => ReturnCode::ServerDisablesAt,
            10027 => ReturnCode::ClientDisablesAt,
            10028 => ReturnCode::LOCKED,
            10029 => ReturnCode::FROZEN,
            10030 => ReturnCode::InvalidFill,
            10031 => ReturnCode::CONNECTION,
            10032 => ReturnCode::OnlyReal,
            10033 => ReturnCode::LimitOrders,
            10034 => ReturnCode::LimitVolumes,
            10035 => ReturnCode::InvalidOrders,
            10036 => ReturnCode::PostionClosed,
            10038 => ReturnCode::InvalideCloseVolume,
            10039 => ReturnCode::CloseOrderExist,
            10040 => ReturnCode::LimitPositions,
            10041 => ReturnCode::RejectCancel,
            10042 => ReturnCode::LongOnly,
            10043 => ReturnCode::ShortOnly,
            10044 => ReturnCode::CloseOnly,
            10045 => ReturnCode::FifoClose,
            10046 => ReturnCode::HedgeProhibited,
            _ => panic!("Invalid ReturnCode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for ReturnCode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(ReturnCode::CHECKED),
            10004 => Ok(ReturnCode::REQUOTE),
            10006 => Ok(ReturnCode::REJECT),
            10007 => Ok(ReturnCode::CANCELLED),
            10008 => Ok(ReturnCode::PLACED),
            10009 => Ok(ReturnCode::DONE),
            10010 => Ok(ReturnCode::DonePartial),
            10011 => Ok(ReturnCode::ERROR),
            10012 => Ok(ReturnCode::TIMEOUT),
            10013 => Ok(ReturnCode::INVALID),
            10014 => Ok(ReturnCode::InvalidVolume),
            10015 => Ok(ReturnCode::InvalidPrice),
            10016 => Ok(ReturnCode::InvalidStops),
            10017 => Ok(ReturnCode::TradeDisabled),
            10018 => Ok(ReturnCode::MarketClosed),
            10019 => Ok(ReturnCode::NoMoney),
            10020 => Ok(ReturnCode::PriceChanged),
            10021 => Ok(ReturnCode::PriceOff),
            10022 => Ok(ReturnCode::InvalidExpiration),
            10023 => Ok(ReturnCode::OrderChanged),
            10024 => Ok(ReturnCode::TooManyRequest),
            10025 => Ok(ReturnCode::NoChanges),
            10026 => Ok(ReturnCode::ServerDisablesAt),
            10027 => Ok(ReturnCode::ClientDisablesAt),
            10028 => Ok(ReturnCode::LOCKED),
            10029 => Ok(ReturnCode::FROZEN),
            10030 => Ok(ReturnCode::InvalidFill),
            10031 => Ok(ReturnCode::CONNECTION),
            10032 => Ok(ReturnCode::OnlyReal),
            10033 => Ok(ReturnCode::LimitOrders),
            10034 => Ok(ReturnCode::LimitVolumes),
            10035 => Ok(ReturnCode::InvalidOrders),
            10036 => Ok(ReturnCode::PostionClosed),
            10038 => Ok(ReturnCode::InvalideCloseVolume),
            10039 => Ok(ReturnCode::CloseOrderExist),
            10040 => Ok(ReturnCode::LimitPositions),
            10041 => Ok(ReturnCode::RejectCancel),
            10042 => Ok(ReturnCode::LongOnly),
            10043 => Ok(ReturnCode::ShortOnly),
            10044 => Ok(ReturnCode::CloseOnly),
            10045 => Ok(ReturnCode::FifoClose),
            10046 => Ok(ReturnCode::HedgeProhibited),
            _ => panic!("Invalid ReturnCode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum PositionType {
    BUY = 0,
    SELL = 1,
}

impl From<u64> for PositionType {
    fn from(value: u64) -> Self {
        match value {
            0 => PositionType::BUY,
            1 => PositionType::SELL,
            _ => panic!("Invalid PositionType value: {}", value),
        }
    }
}

impl FromPyObject<'_> for PositionType {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(PositionType::BUY),
            1 => Ok(PositionType::SELL),
            _ => panic!("Invalid PositionType value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum PositionReason {
    CLIENT = 0,
    MOBILE = 1,
    WEB = 2,
    EXPERT = 3,
}

impl From<u64> for PositionReason {
    fn from(value: u64) -> Self {
        match value {
            0 => PositionReason::CLIENT,
            1 => PositionReason::MOBILE,
            2 => PositionReason::WEB,
            3 => PositionReason::EXPERT,
            _ => panic!("Invalid PositionReason value: {}", value),
        }
    }
}

impl FromPyObject<'_> for PositionReason {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(PositionReason::CLIENT),
            1 => Ok(PositionReason::MOBILE),
            2 => Ok(PositionReason::WEB),
            3 => Ok(PositionReason::EXPERT),
            _ => panic!("Invalid PositionReason value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum DealType {
    BUY = 0,
    SELL = 1,
    BALANCE = 2,
    CREDIT = 3,
    CHARGE = 4,
    CORRECTION = 5,
    BONUS = 6,
    COMMISSION = 7,
    CommissionDaily = 8,
    CommissionMontly = 9,
    ComissionAgentDaily = 10,
    CommisionAgentMontly = 11,
    INTEREST = 12,
    BuyCanceled = 13,
    SellCanceled = 14,
    DIVIDEND = 15,
    DividentFranked = 16,
    TAX = 17,
}

impl From<u64> for DealType {
    fn from(value: u64) -> Self {
        match value {
            0 => DealType::BUY,
            1 => DealType::SELL,
            2 => DealType::BALANCE,
            3 => DealType::CREDIT,
            4 => DealType::CHARGE,
            5 => DealType::CORRECTION,
            6 => DealType::BONUS,
            7 => DealType::COMMISSION,
            8 => DealType::CommissionDaily,
            9 => DealType::CommissionMontly,
            10 => DealType::ComissionAgentDaily,
            11 => DealType::CommisionAgentMontly,
            12 => DealType::INTEREST,
            13 => DealType::BuyCanceled,
            14 => DealType::SellCanceled,
            15 => DealType::DIVIDEND,
            16 => DealType::DividentFranked,
            17 => DealType::TAX,
            _ => panic!("Invalid DealType value: {}", value),
        }
    }
}

impl FromPyObject<'_> for DealType {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(DealType::BUY),
            1 => Ok(DealType::SELL),
            2 => Ok(DealType::BALANCE),
            3 => Ok(DealType::CREDIT),
            4 => Ok(DealType::CHARGE),
            5 => Ok(DealType::CORRECTION),
            6 => Ok(DealType::BONUS),
            7 => Ok(DealType::COMMISSION),
            8 => Ok(DealType::CommissionDaily),
            9 => Ok(DealType::CommissionMontly),
            10 => Ok(DealType::ComissionAgentDaily),
            11 => Ok(DealType::CommisionAgentMontly),
            12 => Ok(DealType::INTEREST),
            13 => Ok(DealType::BuyCanceled),
            14 => Ok(DealType::SellCanceled),
            15 => Ok(DealType::DIVIDEND),
            16 => Ok(DealType::DividentFranked),
            17 => Ok(DealType::TAX),
            _ => panic!("Invalid DealType value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum DealEntry {
    IN = 0,
    OUT = 1,
    INOUT = 2,
    OutBy = 3,
}

impl From<u64> for DealEntry {
    fn from(value: u64) -> Self {
        match value {
            0 => DealEntry::IN,
            1 => DealEntry::OUT,
            2 => DealEntry::INOUT,
            3 => DealEntry::OutBy,
            _ => panic!("Invalid DealEntry value: {}", value),
        }
    }
}

impl FromPyObject<'_> for DealEntry {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(DealEntry::IN),
            1 => Ok(DealEntry::OUT),
            2 => Ok(DealEntry::INOUT),
            3 => Ok(DealEntry::OutBy),
            _ => panic!("Invalid DealEntry value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum DealReason {
    CLIENT = 0,
    MOBILE = 1,
    WEB = 2,
    EXPERT = 3,
    SL = 4,
    TP = 5,
    SO = 6,
    ROLLOVER = 7,
    VMARGIN = 8,
    SPLIT = 9,
}

impl From<u64> for DealReason {
    fn from(value: u64) -> Self {
        match value {
            0 => DealReason::CLIENT,
            1 => DealReason::MOBILE,
            2 => DealReason::WEB,
            3 => DealReason::EXPERT,
            4 => DealReason::SL,
            5 => DealReason::TP,
            6 => DealReason::SO,
            7 => DealReason::ROLLOVER,
            8 => DealReason::VMARGIN,
            9 => DealReason::SPLIT,
            _ => panic!("Invalid DealReason value: {}", value),
        }
    }
}

impl FromPyObject<'_> for DealReason {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(DealReason::CLIENT),
            1 => Ok(DealReason::MOBILE),
            2 => Ok(DealReason::WEB),
            3 => Ok(DealReason::EXPERT),
            4 => Ok(DealReason::SL),
            5 => Ok(DealReason::TP),
            6 => Ok(DealReason::SO),
            7 => Ok(DealReason::ROLLOVER),
            8 => Ok(DealReason::VMARGIN),
            9 => Ok(DealReason::SPLIT),
            _ => panic!("Invalid DealReason value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum OrderState {
    STARTED = 0,
    PLACED = 1,
    CANCELED = 2,
    PARTIAL = 3,
    FILLED = 4,
    REJECTED = 5,
    EXPIRED = 6,
    RequestAdd = 7,
    RequestModify = 8,
    RequestCancel = 9,
}

impl From<u64> for OrderState {
    fn from(value: u64) -> Self {
        match value {
            0 => OrderState::STARTED,
            1 => OrderState::PLACED,
            2 => OrderState::CANCELED,
            3 => OrderState::PARTIAL,
            4 => OrderState::FILLED,
            5 => OrderState::REJECTED,
            6 => OrderState::EXPIRED,
            7 => OrderState::RequestAdd,
            8 => OrderState::RequestModify,
            9 => OrderState::RequestCancel,
            _ => panic!("Invalid OrderState value: {}", value),
        }
    }
}

impl FromPyObject<'_> for OrderState {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(OrderState::STARTED),
            1 => Ok(OrderState::PLACED),
            2 => Ok(OrderState::CANCELED),
            3 => Ok(OrderState::PARTIAL),
            4 => Ok(OrderState::FILLED),
            5 => Ok(OrderState::REJECTED),
            6 => Ok(OrderState::EXPIRED),
            7 => Ok(OrderState::RequestAdd),
            8 => Ok(OrderState::RequestModify),
            9 => Ok(OrderState::RequestCancel),
            _ => panic!("Invalid OrderState value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum OrderReason {
    CLIENT = 0,
    MOBILE = 1,
    WEB = 2,
    EXPERT = 3,
    SL = 4,
    TP = 5,
    SO = 6,
}

impl From<u64> for OrderReason {
    fn from(value: u64) -> Self {
        match value {
            0 => OrderReason::CLIENT,
            1 => OrderReason::MOBILE,
            2 => OrderReason::WEB,
            3 => OrderReason::EXPERT,
            4 => OrderReason::SL,
            5 => OrderReason::TP,
            6 => OrderReason::SO,
            _ => panic!("Invalid OrderReason value: {}", value),
        }
    }
}

impl FromPyObject<'_> for OrderReason {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(OrderReason::CLIENT),
            1 => Ok(OrderReason::MOBILE),
            2 => Ok(OrderReason::WEB),
            3 => Ok(OrderReason::EXPERT),
            4 => Ok(OrderReason::SL),
            5 => Ok(OrderReason::TP),
            6 => Ok(OrderReason::SO),
            _ => panic!("Invalid OrderReason value: {}", value),
        }
    }
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

pub enum SymbolInfoProperty {
    Custom,
    ChartMode,
    Select,
    Visible,
    SessionDeals,
    SessionBuyOrders,
    SessionSellOrders,
    Volume,
    VolumeHigh,
    VolumeLow,
    Time,
    Digits,
    Spread,
    SpreadFloat,
    TicksBookDepth,
    TradeCalcMode,
    TradeMode,
    StartTime,
    ExpirationTime,
    TradeStopsLevel,
    TradeFreezeLevel,
    TradeExeMode,
    SwapMode,
    SwapRollover3Days,
    MarginHedgedUseLeg,
    ExpirationMode,
    FillingMode,
    OrderMode,
    OrderGtcMode,
    OptionMode,
    OptionRight,
    Bid,
    BidHigh,
    BidLow,
    Ask,
    AskHigh,
    AskLow,
    Last,
    LastHigh,
    LastLow,
    VolumeReal,
    VolumeHighReal,
    VolumeLowReal,
    OptionStrike,
    Point,
    TradeTickValue,
    TradeTickValueProfit,
    TradeTickValueLoss,
    TradeTickSize,
    TradeContractSize,
    TradeAccruedInterest,
    TradeFaceValue,
    TradeLiquidityRate,
    VolumeMin,
    VolumeMax,
    VolumeStep,
    VolumeLimit,
    SwapLong,
    SwapShort,
    MarginInitial,
    MarginMaintenance,
    SessionVolume,
    SessionTurnover,
    SessionInterest,
    SessionBuyOrdersVolume,
    SessionSellOrdersVolume,
    SessionOpen,
    SessionClose,
    SessionAw,
    SessionPriceSettlement,
    SessionPriceLimitMin,
    SessionPriceLimitMax,
    MarginHedged,
    PriceChange,
    PriceVolatility,
    PriceTheoretical,
    PriceGreeksDelta,
    PriceGreeksTheta,
    PriceGreeksGamma,
    PriceGreeksVega,
    PriceGreeksRho,
    PriceGreeksOmega,
    PriceSensitivity,
    Basis,
    Category,
    CurrencyBase,
    CurrencyProfit,
    CurrencyMargin,
    Bank,
    Description,
    Exchange,
    Formula,
    Isin,
    Name,
    Page,
    Path,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum AccountTradeMode {
    Demo = 0,
    Contest = 1,
    Real = 2,
}

impl From<u64> for AccountTradeMode {
    fn from(value: u64) -> Self {
        match value {
            0 => AccountTradeMode::Demo,
            1 => AccountTradeMode::Contest,
            2 => AccountTradeMode::Real,
            _ => panic!("Invalid AccountTradeMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for AccountTradeMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(AccountTradeMode::Demo),
            1 => Ok(AccountTradeMode::Contest),
            2 => Ok(AccountTradeMode::Real),
            _ => panic!("Invalid AccountTradeMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum AccountStopOutMode {
    PERCENT = 0,
    MONEY = 1,
}

impl From<u64> for AccountStopOutMode {
    fn from(value: u64) -> Self {
        match value {
            0 => AccountStopOutMode::PERCENT,
            1 => AccountStopOutMode::MONEY,
            _ => panic!("Invalid AccountStopOutMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for AccountStopOutMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(AccountStopOutMode::PERCENT),
            1 => Ok(AccountStopOutMode::MONEY),
            _ => panic!("Invalid AccountStopOutMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum AccountMarginMode {
    RetailNetting = 0,
    Exchange = 1,
    RetailHedging = 2,
}

impl From<u64> for AccountMarginMode {
    fn from(value: u64) -> Self {
        match value {
            0 => AccountMarginMode::RetailNetting,
            1 => AccountMarginMode::Exchange,
            2 => AccountMarginMode::RetailHedging,
            _ => panic!("Invalid AccountMarginMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for AccountMarginMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(AccountMarginMode::RetailNetting),
            1 => Ok(AccountMarginMode::Exchange),
            2 => Ok(AccountMarginMode::RetailHedging),
            _ => panic!("Invalid AccountMarginMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolChartMode {
    Bid = 0,
    Last = 1,
}

impl From<u64> for SymbolChartMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolChartMode::Bid,
            1 => SymbolChartMode::Last,
            _ => panic!("Invalid SymbolChartMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolChartMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolChartMode::Bid),
            1 => Ok(SymbolChartMode::Last),
            _ => panic!("Invalid SymbolChartMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolCalcMode {
    FOREX = 0,
    FUTURES = 1,
    CFD = 2,
    CFDINDEX = 3,
    CFDLEVERAGE = 4,
    ForexNoLeverage = 5,
    ExchStocks = 32,
    ExchFutures = 33,
    ExchOptions = 34,
    ExchOptionsMargin = 36,
    ExchBonds = 37,
    ExchStocksMoex = 38,
    ExchBondsMoex = 39,
    ServCollateral = 64,
}

impl From<u64> for SymbolCalcMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolCalcMode::FOREX,
            1 => SymbolCalcMode::FUTURES,
            2 => SymbolCalcMode::CFD,
            3 => SymbolCalcMode::CFDINDEX,
            4 => SymbolCalcMode::CFDLEVERAGE,
            5 => SymbolCalcMode::ForexNoLeverage,
            32 => SymbolCalcMode::ExchStocks,
            33 => SymbolCalcMode::ExchFutures,
            34 => SymbolCalcMode::ExchOptions,
            36 => SymbolCalcMode::ExchOptionsMargin,
            37 => SymbolCalcMode::ExchBonds,
            38 => SymbolCalcMode::ExchStocksMoex,
            39 => SymbolCalcMode::ExchBondsMoex,
            64 => SymbolCalcMode::ServCollateral,
            _ => panic!("Invalid SymbolCalcMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolCalcMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolCalcMode::FOREX),
            1 => Ok(SymbolCalcMode::FUTURES),
            2 => Ok(SymbolCalcMode::CFD),
            3 => Ok(SymbolCalcMode::CFDINDEX),
            4 => Ok(SymbolCalcMode::CFDLEVERAGE),
            5 => Ok(SymbolCalcMode::ForexNoLeverage),
            32 => Ok(SymbolCalcMode::ExchStocks),
            33 => Ok(SymbolCalcMode::ExchFutures),
            34 => Ok(SymbolCalcMode::ExchOptions),
            36 => Ok(SymbolCalcMode::ExchOptionsMargin),
            37 => Ok(SymbolCalcMode::ExchBonds),
            38 => Ok(SymbolCalcMode::ExchStocksMoex),
            39 => Ok(SymbolCalcMode::ExchBondsMoex),
            64 => Ok(SymbolCalcMode::ServCollateral),
            _ => panic!("Invalid SymbolCalcMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolTradeMode {
    SymbolTradeModeDisabled = 0,
    SymbolTradeModeLongonly = 1,
    SymbolTradeModeShortonly = 2,
    SymbolTradeModeCloseonly = 3,
    SymbolTradeModeFull = 4,
}

impl From<u64> for SymbolTradeMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolTradeMode::SymbolTradeModeDisabled,
            1 => SymbolTradeMode::SymbolTradeModeLongonly,
            2 => SymbolTradeMode::SymbolTradeModeShortonly,
            3 => SymbolTradeMode::SymbolTradeModeCloseonly,
            4 => SymbolTradeMode::SymbolTradeModeFull,
            _ => panic!("Invalid SymbolTradeMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolTradeMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolTradeMode::SymbolTradeModeDisabled),
            1 => Ok(SymbolTradeMode::SymbolTradeModeLongonly),
            2 => Ok(SymbolTradeMode::SymbolTradeModeShortonly),
            3 => Ok(SymbolTradeMode::SymbolTradeModeCloseonly),
            4 => Ok(SymbolTradeMode::SymbolTradeModeFull),
            _ => panic!("Invalid SymbolTradeMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolTradeExecution {
    Request = 0,
    Instant = 1,
    Market = 2,
    Exchange = 3,
}

impl From<u64> for SymbolTradeExecution {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolTradeExecution::Request,
            1 => SymbolTradeExecution::Instant,
            2 => SymbolTradeExecution::Market,
            3 => SymbolTradeExecution::Exchange,
            _ => panic!("Invalid SymbolTradeExecution value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolTradeExecution {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolTradeExecution::Request),
            1 => Ok(SymbolTradeExecution::Instant),
            2 => Ok(SymbolTradeExecution::Market),
            3 => Ok(SymbolTradeExecution::Exchange),
            _ => panic!("Invalid SymbolTradeExecution value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolSwapMode {
    Disabled = 0,
    Points = 1,
    CurrencySymbol = 2,
    CurrencyMargin = 3,
    CurrencyDeposit = 4,
    InterestCurrent = 5,
    InterestOpen = 6,
    ReopenCurrent = 7,
    ReopenBid = 8,
}

impl From<u64> for SymbolSwapMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolSwapMode::Disabled,
            1 => SymbolSwapMode::Points,
            2 => SymbolSwapMode::CurrencySymbol,
            3 => SymbolSwapMode::CurrencyMargin,
            4 => SymbolSwapMode::CurrencyDeposit,
            5 => SymbolSwapMode::InterestCurrent,
            6 => SymbolSwapMode::InterestOpen,
            7 => SymbolSwapMode::ReopenCurrent,
            8 => SymbolSwapMode::ReopenBid,
            _ => panic!("Invalid SymbolSwapMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolSwapMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolSwapMode::Disabled),
            1 => Ok(SymbolSwapMode::Points),
            2 => Ok(SymbolSwapMode::CurrencySymbol),
            3 => Ok(SymbolSwapMode::CurrencyMargin),
            4 => Ok(SymbolSwapMode::CurrencyDeposit),
            5 => Ok(SymbolSwapMode::InterestCurrent),
            6 => Ok(SymbolSwapMode::InterestOpen),
            7 => Ok(SymbolSwapMode::ReopenCurrent),
            8 => Ok(SymbolSwapMode::ReopenBid),
            _ => panic!("Invalid SymbolSwapMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    All = 7,
}

impl From<u64> for DayOfWeek {
    fn from(value: u64) -> Self {
        match value {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            7 => DayOfWeek::All,
            _ => panic!("Invalid DayOfWeek value: {}", value),
        }
    }
}

impl FromPyObject<'_> for DayOfWeek {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(DayOfWeek::Sunday),
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            7 => Ok(DayOfWeek::All),
            _ => panic!("Invalid DayOfWeek value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolOrderGtcMode {
    Gtc = 0,
    Daily = 1,
    DailyNoStops = 2,
}

impl From<u64> for SymbolOrderGtcMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolOrderGtcMode::Gtc,
            1 => SymbolOrderGtcMode::Daily,
            2 => SymbolOrderGtcMode::DailyNoStops,
            _ => panic!("Invalid SymbolOrderGtcMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolOrderGtcMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolOrderGtcMode::Gtc),
            1 => Ok(SymbolOrderGtcMode::Daily),
            2 => Ok(SymbolOrderGtcMode::DailyNoStops),
            _ => panic!("Invalid SymbolOrderGtcMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolOptionRight {
    Call = 0,
    Put = 1,
}

impl From<u64> for SymbolOptionRight {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolOptionRight::Call,
            1 => SymbolOptionRight::Put,
            _ => panic!("Invalid SymbolOptionRight value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolOptionRight {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolOptionRight::Call),
            1 => Ok(SymbolOptionRight::Put),
            _ => panic!("Invalid SymbolOptionRight value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolOptionMode {
    European = 0,
    American = 1,
}

impl From<u64> for SymbolOptionMode {
    fn from(value: u64) -> Self {
        match value {
            0 => SymbolOptionMode::European,
            1 => SymbolOptionMode::American,
            _ => panic!("Invalid SymbolOptionMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolOptionMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            0 => Ok(SymbolOptionMode::European),
            1 => Ok(SymbolOptionMode::American),
            _ => panic!("Invalid SymbolOptionMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolExpirationMode {
    Gtc = 1,
    Day = 2,
    Specified = 4,
    SpecifiedDay = 8,
    All = 15,
}

impl From<u64> for SymbolExpirationMode {
    fn from(value: u64) -> Self {
        match value {
            1 => SymbolExpirationMode::Gtc,
            2 => SymbolExpirationMode::Day,
            4 => SymbolExpirationMode::Specified,
            8 => SymbolExpirationMode::SpecifiedDay,
            15 => SymbolExpirationMode::All,
            _ => panic!("Invalid SymbolExpirationMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolExpirationMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            1 => Ok(SymbolExpirationMode::Gtc),
            2 => Ok(SymbolExpirationMode::Day),
            4 => Ok(SymbolExpirationMode::Specified),
            8 => Ok(SymbolExpirationMode::SpecifiedDay),
            15 => Ok(SymbolExpirationMode::All),
            _ => panic!("Invalid SymbolExpirationMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolFillingMode {
    Fok = 1,
    Ioc = 2,
    Boc = 3,
}

impl From<u64> for SymbolFillingMode {
    fn from(value: u64) -> Self {
        match value {
            1 => SymbolFillingMode::Fok,
            2 => SymbolFillingMode::Ioc,
            3 => SymbolFillingMode::Boc,
            _ => panic!("Invalid SymbolFillingMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolFillingMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            1 => Ok(SymbolFillingMode::Fok),
            2 => Ok(SymbolFillingMode::Ioc),
            3 => Ok(SymbolFillingMode::Boc),
            _ => panic!("Invalid SymbolFillingMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum SymbolOrderMode {
    Market = 1,
    Limit = 2,
    Stop = 4,
    StopLimit = 8,
    Sl = 16,
    Tp = 32,
    CloseBy = 64,
    All = 127,
}

impl From<u64> for SymbolOrderMode {
    fn from(value: u64) -> Self {
        match value {
            1 => SymbolOrderMode::Market,
            2 => SymbolOrderMode::Limit,
            4 => SymbolOrderMode::Stop,
            8 => SymbolOrderMode::StopLimit,
            16 => SymbolOrderMode::Sl,
            32 => SymbolOrderMode::Tp,
            64 => SymbolOrderMode::CloseBy,
            127 => SymbolOrderMode::All,
            _ => panic!("Invalid SymbolOrderMode value: {}", value),
        }
    }
}

impl FromPyObject<'_> for SymbolOrderMode {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: u64 = ob.extract().unwrap();
        match value {
            1 => Ok(SymbolOrderMode::Market),
            2 => Ok(SymbolOrderMode::Limit),
            4 => Ok(SymbolOrderMode::Stop),
            8 => Ok(SymbolOrderMode::StopLimit),
            16 => Ok(SymbolOrderMode::Sl),
            32 => Ok(SymbolOrderMode::Tp),
            64 => Ok(SymbolOrderMode::CloseBy),
            127 => Ok(SymbolOrderMode::All),
            _ => panic!("Invalid SymbolOrderMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum RuntimeError {
    Ok = 1,
    Fail = -1,
    InvalidParams = -2,
    NoMemory = -3,
    NotFound = -4,
    InvalidVersion = -5,
    AuthFailed = -6,
    Unsupported = -7,
    AutoTradingDisabled = -8,
    InternalFail = -10000,
    InternalFailSend = -10001,
    InternalFailReceive = -10002,
    InternalFailInit = -10003,
    InternalFailTimeout = -10005,
}

impl From<i64> for RuntimeError {
    fn from(value: i64) -> Self {
        match value {
            1 => RuntimeError::Ok,
            -1 => RuntimeError::Fail,
            -2 => RuntimeError::InvalidParams,
            -3 => RuntimeError::NoMemory,
            -4 => RuntimeError::NotFound,
            -5 => RuntimeError::InvalidVersion,
            -6 => RuntimeError::AuthFailed,
            -7 => RuntimeError::Unsupported,
            -8 => RuntimeError::AutoTradingDisabled,
            -10000 => RuntimeError::InternalFail,
            -10001 => RuntimeError::InternalFailSend,
            -10002 => RuntimeError::InternalFailReceive,
            -10003 => RuntimeError::InternalFailInit,
            -10005 => RuntimeError::InternalFailTimeout,
            _ => panic!("Invalid RuntimeError value: {}", value),
        }
    }
}

impl FromPyObject<'_> for RuntimeError {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();
        match value {
            1 => Ok(RuntimeError::Ok),
            -1 => Ok(RuntimeError::Fail),
            -2 => Ok(RuntimeError::InvalidParams),
            -3 => Ok(RuntimeError::NoMemory),
            -4 => Ok(RuntimeError::NotFound),
            -5 => Ok(RuntimeError::InvalidVersion),
            -6 => Ok(RuntimeError::AuthFailed),
            -7 => Ok(RuntimeError::Unsupported),
            -8 => Ok(RuntimeError::AutoTradingDisabled),
            -10000 => Ok(RuntimeError::InternalFail),
            -10001 => Ok(RuntimeError::InternalFailSend),
            -10002 => Ok(RuntimeError::InternalFailReceive),
            -10003 => Ok(RuntimeError::InternalFailInit),
            -10005 => Ok(RuntimeError::InternalFailTimeout),
            _ => panic!("Invalid RuntimeError value: {}", value),
        }
    }
}
