use core::panic;

pub enum MQLTimeframe {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M6 = 6,
    M10 = 10,
    M12 = 12,
    M15 = 15,
    M20 = 20,
    M30 = 30,
    H1 = 1 | 0x4000,
    H2 = 2 | 0x4000,
    H4 = 4 | 0x4000,
    H3 = 3 | 0x4000,
    H6 = 6 | 0x4000,
    H8 = 8 | 0x4000,
    H12 = 12 | 0x4000,
    D1 = 24 | 0x4000,
    W1 = 1 | 0x8000,
    MN1 = 1 | 0xC000,
}

pub enum MQLCopyTicksFlags {
    ALL = -1,
    INFO = 1,
    TRADE = 2,
}

pub enum MQLTicksFlag {
    BID = 0x02,
    ASK = 0x04,
    LAST = 0x08,
    VOLUME = 0x10,
    BUY = 0x20,
    SELL = 0x40,
}

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

pub enum TradeActionRequest {
    DEAL = 1,
    PENDING = 5,
    SLTP = 6,
    MODIFY = 7,
    REMOVE = 8,
    CloseBy = 10,
}

impl From<i64> for TradeActionRequest {
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

pub enum MQLTradeOrderTypeFilling {
    FOK = 0,
    IOC = 1,
    RETURN = 2,
}

pub enum MQLTradeOrderTypeTime {
    GTC = 0,
    DAY = 1,
    SPECIFIED = 2,
    SpecifiedDay = 3,
}

pub enum OrderTypeFilling {
    FOK = 0,
    IOC = 1,
    RETURN = 2,
}

impl From<i64> for OrderTypeFilling {
    fn from(value: i64) -> Self {
        match value {
            0 => OrderTypeFilling::FOK,
            1 => OrderTypeFilling::IOC,
            2 => OrderTypeFilling::RETURN,
            _ => panic!("Invalid OrderTypeFilling value: {}", value),
        }
    }
}

pub enum OrderTypeTime {
    GTC = 0,
    DAY = 1,
    SPECIFIED = 2,
    SpecifiedDay = 3,
}

impl From<i64> for OrderTypeTime {
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
