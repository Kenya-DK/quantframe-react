use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
#[derive(Eq)]
pub enum StockStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "live")]
    Live,
    #[sea_orm(string_value = "to_low_profit")]
    ToLowProfit,
    #[sea_orm(string_value = "no_sellers")]
    NoSellers,
    #[sea_orm(string_value = "no_buyers")]
    NoBuyers,
    #[sea_orm(string_value = "inactive")]
    InActive,
    #[sea_orm(string_value = "sma_limit")]
    SMALimit,
    #[sea_orm(string_value = "order_limit")]
    OrderLimit,
    #[sea_orm(string_value = "overpriced")]
    Overpriced,
    #[sea_orm(string_value = "underpriced")]
    Underpriced,
}
impl StockStatus {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            StockStatus::Pending => "pending",
            StockStatus::Live => "live",
            StockStatus::ToLowProfit => "to_low_profit",
            StockStatus::NoSellers => "no_sellers",
            StockStatus::NoBuyers => "no_buyers",
            StockStatus::InActive => "inactive",
            StockStatus::SMALimit => "sma_limit",
            StockStatus::OrderLimit => "order_limit",
            StockStatus::Overpriced => "overpriced",
            StockStatus::Underpriced => "underpriced",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            StockStatus::Pending => StockStatus::Pending.as_str().to_string(),
            StockStatus::Live => StockStatus::Live.as_str().to_string(),
            StockStatus::ToLowProfit => StockStatus::ToLowProfit.as_str().to_string(),
            StockStatus::NoSellers => StockStatus::NoSellers.as_str().to_string(),
            StockStatus::NoBuyers => StockStatus::NoBuyers.as_str().to_string(),
            StockStatus::InActive => StockStatus::InActive.as_str().to_string(),
            StockStatus::SMALimit => StockStatus::SMALimit.as_str().to_string(),
            StockStatus::OrderLimit => StockStatus::OrderLimit.as_str().to_string(),
            StockStatus::Overpriced => StockStatus::Overpriced.as_str().to_string(),
            StockStatus::Underpriced => StockStatus::Underpriced.as_str().to_string(),
        }
    }
    pub fn from_string(s: &str) -> StockStatus {
        match s {
            "pending" => StockStatus::Pending,
            "live" => StockStatus::Live,
            "to_low_profit" => StockStatus::ToLowProfit,
            "no_sellers" => StockStatus::NoSellers,
            "no_buyers" => StockStatus::NoBuyers,
            "inactive" => StockStatus::InActive,
            "sma_limit" => StockStatus::SMALimit,
            "order_limit" => StockStatus::OrderLimit,
            "overpriced" => StockStatus::Overpriced,
            "underpriced" => StockStatus::Underpriced,
            _ => StockStatus::Pending,
        }
    }
}
impl Serialize for StockStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            StockStatus::Pending => StockStatus::Pending.as_str(),
            StockStatus::Live => StockStatus::Live.as_str(),
            StockStatus::ToLowProfit => StockStatus::ToLowProfit.as_str(),
            StockStatus::NoSellers => StockStatus::NoSellers.as_str(),
            StockStatus::NoBuyers => StockStatus::NoBuyers.as_str(),
            StockStatus::InActive => StockStatus::InActive.as_str(),
            StockStatus::SMALimit => StockStatus::SMALimit.as_str(),
            StockStatus::OrderLimit => StockStatus::OrderLimit.as_str(),
            StockStatus::Overpriced => StockStatus::Overpriced.as_str(),
            StockStatus::Underpriced => StockStatus::Underpriced.as_str(),
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for StockStatus {
    fn deserialize<D>(deserializer: D) -> Result<StockStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pending" => StockStatus::Pending,
            "live" => StockStatus::Live,
            "to_low_profit" => StockStatus::ToLowProfit,
            "no_sellers" => StockStatus::NoSellers,
            "no_buyers" => StockStatus::NoBuyers,
            "inactive" => StockStatus::InActive,
            "sma_limit" => StockStatus::SMALimit,
            "order_limit" => StockStatus::OrderLimit,
            "overpriced" => StockStatus::Overpriced,
            "underpriced" => StockStatus::Underpriced,
            _ => StockStatus::Pending,
        })
    }
}

impl std::hash::Hash for StockStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}
