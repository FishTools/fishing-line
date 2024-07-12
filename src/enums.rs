/// Represents the timeframe for a trading operation.
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
pub enum CopyTicksFlags {
    ALL = -1,
    INFO = 1,
    TRADE = 2,
}

/// Represents the flags for ticks.
pub enum TicksFlag {
    BID = 0x02,
    ASK = 0x04,
    LAST = 0x08,
    VOLUME = 0x10,
    BUY = 0x20,
    SELL = 0x40,
}

/// Represents the type of an order.
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
pub enum TradeActionRequest {
    DEAL = 1,
    PENDING = 5,
    SLTP = 6,
    MODIFY = 7,
    REMOVE = 8,
    CloseBy = 10,
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

/// Represents the filling type for a trade order.
pub enum TradeOrderTypeFilling {
    FOK = 0,
    IOC = 1,
    RETURN = 2,
}

/// Represents the time type for a trade order.
pub enum TradeOrderTypeTime {
    GTC = 0,
    DAY = 1,
    SPECIFIED = 2,
    SpecifiedDay = 3,
}

/// Represents the filling type for an order.
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

/// Represents the time type for an order.
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
