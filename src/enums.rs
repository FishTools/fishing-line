use pyo3::{types::PyAnyMethods, FromPyObject};
use serde::Deserialize;

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
            0 => {
                return Ok(OrderType::BUY);
            }
            1 => {
                return Ok(OrderType::SELL);
            }
            2 => {
                return Ok(OrderType::BuyLimit);
            }
            3 => {
                return Ok(OrderType::SellLimit);
            }
            4 => {
                return Ok(OrderType::BuyStop);
            }
            5 => {
                return Ok(OrderType::SellStop);
            }
            6 => {
                return Ok(OrderType::BuyStopLimit);
            }
            7 => {
                return Ok(OrderType::SellStopLimit);
            }
            8 => {
                return Ok(OrderType::CloseBy);
            }
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
            1 => {
                return Ok(TradeActionRequest::DEAL);
            }
            5 => {
                return Ok(TradeActionRequest::PENDING);
            }
            6 => {
                return Ok(TradeActionRequest::SLTP);
            }
            7 => {
                return Ok(TradeActionRequest::MODIFY);
            }
            8 => {
                return Ok(TradeActionRequest::REMOVE);
            }
            10 => {
                return Ok(TradeActionRequest::CloseBy);
            }
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
            0 => return OrderTypeFilling::FOK,
            1 => return OrderTypeFilling::IOC,
            2 => return OrderTypeFilling::RETURN,
            _ => panic!("Invalid OrderTypeFilling value: {}", value),
        }
    }
}

impl FromPyObject<'_> for OrderTypeFilling {
    fn extract_bound(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<Self> {
        let value: i64 = ob.extract().unwrap();

        match value {
            0 => {
                return Ok(OrderTypeFilling::FOK);
            }
            1 => {
                return Ok(OrderTypeFilling::IOC);
            }
            2 => {
                return Ok(OrderTypeFilling::RETURN);
            }
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
            0 => {
                return Ok(OrderTypeTime::GTC);
            }
            1 => {
                return Ok(OrderTypeTime::DAY);
            }
            2 => {
                return Ok(OrderTypeTime::SPECIFIED);
            }
            3 => {
                return Ok(OrderTypeTime::SpecifiedDay);
            }
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
            0 => {
                return Ok(ReturnCode::CHECKED);
            }
            10004 => {
                return Ok(ReturnCode::REQUOTE);
            }
            10006 => {
                return Ok(ReturnCode::REJECT);
            }
            10007 => {
                return Ok(ReturnCode::CANCELLED);
            }
            10008 => {
                return Ok(ReturnCode::PLACED);
            }
            10009 => {
                return Ok(ReturnCode::DONE);
            }
            10010 => {
                return Ok(ReturnCode::DonePartial);
            }
            10011 => {
                return Ok(ReturnCode::ERROR);
            }
            10012 => {
                return Ok(ReturnCode::TIMEOUT);
            }
            10013 => {
                return Ok(ReturnCode::INVALID);
            }
            10014 => {
                return Ok(ReturnCode::InvalidVolume);
            }
            10015 => {
                return Ok(ReturnCode::InvalidPrice);
            }
            10016 => {
                return Ok(ReturnCode::InvalidStops);
            }
            10017 => {
                return Ok(ReturnCode::TradeDisabled);
            }
            10018 => {
                return Ok(ReturnCode::MarketClosed);
            }
            10019 => {
                return Ok(ReturnCode::NoMoney);
            }
            10020 => {
                return Ok(ReturnCode::PriceChanged);
            }
            10021 => {
                return Ok(ReturnCode::PriceOff);
            }
            10022 => {
                return Ok(ReturnCode::InvalidExpiration);
            }
            10023 => {
                return Ok(ReturnCode::OrderChanged);
            }
            10024 => {
                return Ok(ReturnCode::TooManyRequest);
            }
            10025 => {
                return Ok(ReturnCode::NoChanges);
            }
            10026 => {
                return Ok(ReturnCode::ServerDisablesAt);
            }
            10027 => {
                return Ok(ReturnCode::ClientDisablesAt);
            }
            10028 => {
                return Ok(ReturnCode::LOCKED);
            }
            10029 => {
                return Ok(ReturnCode::FROZEN);
            }
            10030 => {
                return Ok(ReturnCode::InvalidFill);
            }
            10031 => {
                return Ok(ReturnCode::CONNECTION);
            }
            10032 => {
                return Ok(ReturnCode::OnlyReal);
            }
            10033 => {
                return Ok(ReturnCode::LimitOrders);
            }
            10034 => {
                return Ok(ReturnCode::LimitVolumes);
            }
            10035 => {
                return Ok(ReturnCode::InvalidOrders);
            }
            10036 => {
                return Ok(ReturnCode::PostionClosed);
            }
            10038 => {
                return Ok(ReturnCode::InvalideCloseVolume);
            }
            10039 => {
                return Ok(ReturnCode::CloseOrderExist);
            }
            10040 => {
                return Ok(ReturnCode::LimitPositions);
            }
            10041 => {
                return Ok(ReturnCode::RejectCancel);
            }
            10042 => {
                return Ok(ReturnCode::LongOnly);
            }
            10043 => {
                return Ok(ReturnCode::ShortOnly);
            }
            10044 => {
                return Ok(ReturnCode::CloseOnly);
            }
            10045 => {
                return Ok(ReturnCode::FifoClose);
            }
            10046 => {
                return Ok(ReturnCode::HedgeProhibited);
            }
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
            0 => {
                return Ok(PositionType::BUY);
            }
            1 => {
                return Ok(PositionType::SELL);
            }
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
            0 => {
                return Ok(PositionReason::CLIENT);
            }
            1 => {
                return Ok(PositionReason::MOBILE);
            }
            2 => {
                return Ok(PositionReason::WEB);
            }
            3 => {
                return Ok(PositionReason::EXPERT);
            }
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
            0 => {
                return Ok(DealType::BUY);
            }
            1 => {
                return Ok(DealType::SELL);
            }
            2 => {
                return Ok(DealType::BALANCE);
            }
            3 => {
                return Ok(DealType::CREDIT);
            }
            4 => {
                return Ok(DealType::CHARGE);
            }
            5 => {
                return Ok(DealType::CORRECTION);
            }
            6 => {
                return Ok(DealType::BONUS);
            }
            7 => {
                return Ok(DealType::COMMISSION);
            }
            8 => {
                return Ok(DealType::CommissionDaily);
            }
            9 => {
                return Ok(DealType::CommissionMontly);
            }
            10 => {
                return Ok(DealType::ComissionAgentDaily);
            }
            11 => {
                return Ok(DealType::CommisionAgentMontly);
            }
            12 => {
                return Ok(DealType::INTEREST);
            }
            13 => {
                return Ok(DealType::BuyCanceled);
            }
            14 => {
                return Ok(DealType::SellCanceled);
            }
            15 => {
                return Ok(DealType::DIVIDEND);
            }
            16 => {
                return Ok(DealType::DividentFranked);
            }
            17 => {
                return Ok(DealType::TAX);
            }
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
            0 => {
                return Ok(DealEntry::IN);
            }
            1 => {
                return Ok(DealEntry::OUT);
            }
            2 => {
                return Ok(DealEntry::INOUT);
            }
            3 => {
                return Ok(DealEntry::OutBy);
            }
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
            0 => {
                return Ok(DealReason::CLIENT);
            }
            1 => {
                return Ok(DealReason::MOBILE);
            }
            2 => {
                return Ok(DealReason::WEB);
            }
            3 => {
                return Ok(DealReason::EXPERT);
            }
            4 => {
                return Ok(DealReason::SL);
            }
            5 => {
                return Ok(DealReason::TP);
            }
            6 => {
                return Ok(DealReason::SO);
            }
            7 => {
                return Ok(DealReason::ROLLOVER);
            }
            8 => {
                return Ok(DealReason::VMARGIN);
            }
            9 => {
                return Ok(DealReason::SPLIT);
            }
            _ => panic!("Invalid DealReason value: {}", value),
        }
    }
}
