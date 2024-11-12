pub enum ExchangeProps
{
    UsdTwd = 1,
    RmbTwd = 2,
    EurUsd = 3,
    UsdJpy = 4,
    GbpUsd = 5,
    AudUsd = 6,
    UsdHkd = 7,
    UsdRmb = 8,
    UsdZar = 9,
    NzdUsd = 10,
}

impl ExchangeProps {
    pub fn value(&self) -> &str {
        match *self{
            ExchangeProps::UsdTwd => "USD/TWD",
            ExchangeProps::RmbTwd => "RMB/TWD",
            ExchangeProps::EurUsd => "EUR/USD",
            ExchangeProps::UsdJpy => "USD/JPY",
            ExchangeProps::GbpUsd => "GBP/USD",
            ExchangeProps::AudUsd => "AUD/USD",
            ExchangeProps::UsdHkd => "USD/HKD",
            ExchangeProps::UsdRmb => "USD/RMB",
            ExchangeProps::UsdZar => "USD/ZAR",
            ExchangeProps::NzdUsd => "NZD/USD",
        }
    }


    pub fn from_usize(v: usize) -> Option<ExchangeProps> {
        match v{
            x if x == ExchangeProps::UsdTwd as usize => Some(ExchangeProps::UsdTwd),
            x if x == ExchangeProps::RmbTwd as usize => Some(ExchangeProps::RmbTwd),
            x if x == ExchangeProps::EurUsd as usize => Some(ExchangeProps::EurUsd),
            x if x == ExchangeProps::UsdJpy as usize => Some(ExchangeProps::UsdJpy),
            x if x == ExchangeProps::GbpUsd as usize => Some(ExchangeProps::GbpUsd),
            x if x == ExchangeProps::AudUsd as usize => Some(ExchangeProps::AudUsd),
            x if x == ExchangeProps::UsdHkd as usize => Some(ExchangeProps::UsdHkd),
            x if x == ExchangeProps::UsdRmb as usize => Some(ExchangeProps::UsdRmb),
            x if x == ExchangeProps::UsdZar as usize => Some(ExchangeProps::UsdZar),
            x if x == ExchangeProps::NzdUsd as usize => Some(ExchangeProps::NzdUsd),
            _ => None,
        }
    }
}