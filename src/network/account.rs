use crate::schemas;
pub struct AccountInfo {
    url: String,
    access_token: Option<String>,
}

impl AccountInfo {
    pub fn new(url: impl Into<String>) -> AccountInfo {
        AccountInfo {
            url: url.into(),
            access_token: None,
        }
    }

    pub fn authenticate(mut self, credentials: schemas::MQLAccountCredentials) -> Self {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/security/login", self.url);
        let response = client
            .post(url)
            .json(&credentials)
            .send()
            .expect("Unable to process response");
        self.access_token = response.json().expect("Unable to parse JSON");
        return self;
    }

    pub fn info_string(&self, property: schemas::AccountInfoProperty) -> String {
        let account_info: schemas::MQLAccountInfo = self.info_all();

        match property {
            schemas::AccountInfoProperty::Name => account_info.name,
            schemas::AccountInfoProperty::Server => account_info.server,
            schemas::AccountInfoProperty::Currency => account_info.currency,
            schemas::AccountInfoProperty::Company => account_info.company,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_int(&self, property: schemas::AccountInfoProperty) -> i64 {
        let account_info: schemas::MQLAccountInfo = self.info_all();

        match property {
            schemas::AccountInfoProperty::Login => account_info.login,
            schemas::AccountInfoProperty::TradeMode => account_info.trade_mode,
            schemas::AccountInfoProperty::Leverage => account_info.leverage,
            schemas::AccountInfoProperty::LimitOrders => account_info.limit_orders,
            schemas::AccountInfoProperty::MarginSoMode => account_info.margin_so_mode,
            schemas::AccountInfoProperty::MarginMode => account_info.margin_mode,
            schemas::AccountInfoProperty::CurrencyDigits => account_info.currency_digits,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_float(&self, property: schemas::AccountInfoProperty) -> f64 {
        let account_info: schemas::MQLAccountInfo = self.info_all();

        match property {
            schemas::AccountInfoProperty::Balance => account_info.balance,
            schemas::AccountInfoProperty::Credit => account_info.credit,
            schemas::AccountInfoProperty::Profit => account_info.profit,
            schemas::AccountInfoProperty::Equity => account_info.equity,
            schemas::AccountInfoProperty::Margin => account_info.margin,
            schemas::AccountInfoProperty::MarginFree => account_info.margin_free,
            schemas::AccountInfoProperty::MarginLevel => account_info.margin_level,
            schemas::AccountInfoProperty::MarginSoCall => account_info.margin_so_call,
            schemas::AccountInfoProperty::MarginSoSo => account_info.margin_so_so,
            schemas::AccountInfoProperty::MarginInitial => account_info.margin_initial,
            schemas::AccountInfoProperty::MarginMaintenance => account_info.margin_maintenance,
            schemas::AccountInfoProperty::Assets => account_info.assets,
            schemas::AccountInfoProperty::Liabilities => account_info.liabilities,
            schemas::AccountInfoProperty::CommissionBlocked => account_info.commission_blocked,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_bool(&self, property: schemas::AccountInfoProperty) -> bool {
        let account_info: schemas::MQLAccountInfo = self.info_all();

        match property {
            schemas::AccountInfoProperty::TradeAllowed => account_info.trade_allowed,
            schemas::AccountInfoProperty::TradeExpert => account_info.trade_expert,
            schemas::AccountInfoProperty::FifoClose => account_info.fifo_close,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_all(&self) -> schemas::MQLAccountInfo {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/account/", self.url);
        let response = client
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.as_ref().unwrap()),
            )
            .send()
            .expect("Unable to process response");
        let account_info: schemas::MQLAccountInfo = response.json().expect("Unable to parse JSON");
        return account_info;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use schemas::MQLAccountCredentials;

    #[test]
    fn test_account_info() {
        dotenv().ok();
        let account_info = AccountInfo::new("http://localhost:8000");
        let account_info = account_info.authenticate(MQLAccountCredentials {
            login: std::env::var("TERMINAL_ACCOUNT_ID")
                .expect("TERMINAL_ACCOUNT_ID REQUIRED")
                .parse::<i64>()
                .expect("Unable to parse TERMINAL_ACCOUNT_ID"),
            password: std::env::var("TERMINAL_ACCOUNT_PASSWORD")
                .expect("TERMINAL_ACCOUNT_PASSWORD REQUIRED"),
            server: std::env::var("TERMINAL_ACCOUNT_SERVER")
                .expect("TERMINAL_ACCOUNT_SERVER REQUIRED"),
        });
        let account_info = account_info.info_all();
        assert_eq!(account_info.login, 311981450);
    }

    #[test]
    fn test_specific_account_info() {
        dotenv().ok();
        let account_info = AccountInfo::new("http://localhost:8000");
        let account_info = account_info.authenticate(MQLAccountCredentials {
            login: std::env::var("TERMINAL_ACCOUNT_ID")
                .expect("TERMINAL_ACCOUNT_ID REQUIRED")
                .parse::<i64>()
                .expect("Unable to parse TERMINAL_ACCOUNT_ID"),
            password: std::env::var("TERMINAL_ACCOUNT_PASSWORD")
                .expect("TERMINAL_ACCOUNT_PASSWORD REQUIRED"),
            server: std::env::var("TERMINAL_ACCOUNT_SERVER")
                .expect("TERMINAL_ACCOUNT_SERVER REQUIRED"),
        });
        let _login = account_info.info_int(schemas::AccountInfoProperty::Login);
        let _trade_mode = account_info.info_int(schemas::AccountInfoProperty::TradeMode);
        let _leverage = account_info.info_int(schemas::AccountInfoProperty::Leverage);
        let _limit_orders = account_info.info_int(schemas::AccountInfoProperty::LimitOrders);
        let _margin_so_mode = account_info.info_int(schemas::AccountInfoProperty::MarginSoMode);
        let _trade_allowed = account_info.info_bool(schemas::AccountInfoProperty::TradeAllowed);
        let _trade_expert = account_info.info_bool(schemas::AccountInfoProperty::TradeExpert);
        let _margin_mode = account_info.info_int(schemas::AccountInfoProperty::MarginMode);
        let _currency_digits = account_info.info_int(schemas::AccountInfoProperty::CurrencyDigits);
        let _fifo_close = account_info.info_bool(schemas::AccountInfoProperty::FifoClose);
        let _balance = account_info.info_float(schemas::AccountInfoProperty::Balance);
        let _credit = account_info.info_float(schemas::AccountInfoProperty::Credit);
        let _profit = account_info.info_float(schemas::AccountInfoProperty::Profit);
        let _equity = account_info.info_float(schemas::AccountInfoProperty::Equity);
        let _margin = account_info.info_float(schemas::AccountInfoProperty::Margin);
        let _margin_free = account_info.info_float(schemas::AccountInfoProperty::MarginFree);
        let _margin_level = account_info.info_float(schemas::AccountInfoProperty::MarginLevel);
        let _margin_so_call = account_info.info_float(schemas::AccountInfoProperty::MarginSoCall);
        let _margin_so_so = account_info.info_float(schemas::AccountInfoProperty::MarginSoSo);
        let _margin_initial = account_info.info_float(schemas::AccountInfoProperty::MarginInitial);
        let _margin_maintenance =
            account_info.info_float(schemas::AccountInfoProperty::MarginMaintenance);
        let _assets = account_info.info_float(schemas::AccountInfoProperty::Assets);
        let _liabilities = account_info.info_float(schemas::AccountInfoProperty::Liabilities);
        let _commission_blocked =
            account_info.info_float(schemas::AccountInfoProperty::CommissionBlocked);
        let _name = account_info.info_string(schemas::AccountInfoProperty::Name);
        let _server = account_info.info_string(schemas::AccountInfoProperty::Server);
        let _currency = account_info.info_string(schemas::AccountInfoProperty::Currency);
        let _company = account_info.info_string(schemas::AccountInfoProperty::Company);
    }
}
