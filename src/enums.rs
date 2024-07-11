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

pub enum MQLOrderType {
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

pub enum MQLTradeActionRequest {
    DEAL = 1,
    PENDING = 5,
    SLTP = 6,
    MODIFY = 7,
    REMOVE = 8,
    CloseBy = 10,
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
