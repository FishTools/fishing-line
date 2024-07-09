use crate::schemas;

pub struct TerminalInfo {
    url: String,
}

impl TerminalInfo {
    pub fn version(&self) -> schemas::MQLTerminalVersion {
        let url = format!("{}/terminal/version", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");
        let terminal_info: schemas::MQLTerminalVersion =
            response.json().expect("Unable to parse JSON");
        return terminal_info;
    }

    pub fn info_all(&self) -> schemas::MQLTerminalInfo {
        let url = format!("{}/terminal/", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");

        let terminal_info: schemas::MQLTerminalInfo =
            response.json().expect("Unable to parse JSON");
        return terminal_info;
    }

    pub fn info_float(&self, property: schemas::TerminalInfoProperty) -> f64 {
        let url = format!("{}/terminal/", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");
        let terminal_info: schemas::MQLTerminalInfo =
            response.json().expect("Unable to parse JSON");

        match property {
            schemas::TerminalInfoProperty::Retransmission => terminal_info.retransmission,
            schemas::TerminalInfoProperty::CommunityBalance => terminal_info.community_balance,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_int(&self, property: schemas::TerminalInfoProperty) -> i64 {
        let url = format!("{}/terminal/", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");
        let terminal_info: schemas::MQLTerminalInfo =
            response.json().expect("Unable to parse JSON");

        match property {
            schemas::TerminalInfoProperty::Build => terminal_info.build,
            schemas::TerminalInfoProperty::MaxBars => terminal_info.maxbars,
            schemas::TerminalInfoProperty::CodePage => terminal_info.codepage,
            schemas::TerminalInfoProperty::PingLast => terminal_info.ping_last,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_bool(&self, property: schemas::TerminalInfoProperty) -> bool {
        let url = format!("{}/terminal/", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");
        let terminal_info: schemas::MQLTerminalInfo =
            response.json().expect("Unable to parse JSON");

        match property {
            schemas::TerminalInfoProperty::CommunityAccount => terminal_info.community_account,
            schemas::TerminalInfoProperty::CommunityConnection => {
                terminal_info.community_connection
            }
            schemas::TerminalInfoProperty::Connected => terminal_info.connected,
            schemas::TerminalInfoProperty::DllsAllowed => terminal_info.dlls_allowed,
            schemas::TerminalInfoProperty::TradeAllowed => terminal_info.trade_allowed,
            schemas::TerminalInfoProperty::TradeApiDisabled => terminal_info.tradeapi_disabled,
            schemas::TerminalInfoProperty::EmailEnabled => terminal_info.email_enabled,
            schemas::TerminalInfoProperty::FtpEnabled => terminal_info.ftp_enabled,
            schemas::TerminalInfoProperty::NotificationsEnabled => {
                terminal_info.notifications_enabled
            }
            schemas::TerminalInfoProperty::MqId => terminal_info.mqid,
            _ => panic!("Property not found"),
        }
    }

    pub fn info_string(&self, property: schemas::TerminalInfoProperty) -> String {
        let url = format!("{}/terminal/", self.url);
        let response = reqwest::blocking::get(url).expect("Unable to process response");
        let terminal_info: schemas::MQLTerminalInfo =
            response.json().expect("Unable to parse JSON");

        match property {
            schemas::TerminalInfoProperty::Company => terminal_info.company,
            schemas::TerminalInfoProperty::Name => terminal_info.name,
            schemas::TerminalInfoProperty::Language => terminal_info.language,
            schemas::TerminalInfoProperty::Path => terminal_info.path,
            schemas::TerminalInfoProperty::DataPath => terminal_info.data_path,
            schemas::TerminalInfoProperty::CommonDataPath => terminal_info.commondata_path,
            _ => panic!("Property not found"),
        }
    }

    pub fn new(url: impl Into<String>) -> TerminalInfo {
        TerminalInfo { url: url.into() }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_terminal_version() {
        let terminal_info = super::TerminalInfo::new("http://localhost:8000");
        let version = terminal_info.version();
        assert_eq!(version.terminal_version, 500);
        assert_eq!(version.build, 4424);
        assert_eq!(version.build_date, "27 Jun 2024".to_string());
    }

    #[test]
    fn test_get_terminal_info() {
        let terminal_info = super::TerminalInfo::new("http://localhost:8000");
        let info = terminal_info.info_all();
        let version = terminal_info.version();
        assert_eq!(info.build, version.build);
    }

    #[test]
    fn test_get_specific_terminal_info() {
        let terminal_info = super::TerminalInfo::new("http://localhost:8000");
        let _retransmission =
            terminal_info.info_float(super::schemas::TerminalInfoProperty::Retransmission);
        let _community_balance =
            terminal_info.info_float(super::schemas::TerminalInfoProperty::CommunityBalance);
        let _build = terminal_info.info_int(super::schemas::TerminalInfoProperty::Build);
        let _maxbars = terminal_info.info_int(super::schemas::TerminalInfoProperty::MaxBars);
        let _codepage = terminal_info.info_int(super::schemas::TerminalInfoProperty::CodePage);
        let _ping_last = terminal_info.info_int(super::schemas::TerminalInfoProperty::PingLast);
        let _community_account =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::CommunityAccount);
        let _community_connection =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::CommunityConnection);
        let _connected = terminal_info.info_bool(super::schemas::TerminalInfoProperty::Connected);
        let _dlls_allowed =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::DllsAllowed);
        let _trade_allowed =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::TradeAllowed);
        let _tradeapi_disabled =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::TradeApiDisabled);
        let _email_enabled =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::EmailEnabled);
        let _ftp_enabled =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::FtpEnabled);
        let _notifications_enabled =
            terminal_info.info_bool(super::schemas::TerminalInfoProperty::NotificationsEnabled);
        let _mqid = terminal_info.info_bool(super::schemas::TerminalInfoProperty::MqId);
        let _company = terminal_info.info_string(super::schemas::TerminalInfoProperty::Company);
        let _name = terminal_info.info_string(super::schemas::TerminalInfoProperty::Name);
        let _language = terminal_info.info_string(super::schemas::TerminalInfoProperty::Language);
        let _path = terminal_info.info_string(super::schemas::TerminalInfoProperty::Path);
        let _data_path = terminal_info.info_string(super::schemas::TerminalInfoProperty::DataPath);
        let _commondata_path =
            terminal_info.info_string(super::schemas::TerminalInfoProperty::CommonDataPath);
    }
}
