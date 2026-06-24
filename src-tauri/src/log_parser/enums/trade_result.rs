#[derive(Debug, PartialEq, Eq)]
pub enum TradeResult {
    Success,
    Failed,
    Cancelled,
    Unknown,
}
impl TradeResult {
    pub fn was_detected(&self) -> bool {
        matches!(
            self,
            TradeResult::Success | TradeResult::Failed | TradeResult::Cancelled
        )
    }
    pub fn display(&self) -> &'static str {
        match self {
            TradeResult::Success => "Success",
            TradeResult::Failed => "Failed",
            TradeResult::Cancelled => "Cancelled",
            TradeResult::Unknown => "Unknown",
        }
    }
    pub fn metric_name(&self) -> &'static str {
        match self {
            TradeResult::Success => "trade_accepted",
            TradeResult::Failed => "trade_failed",
            TradeResult::Cancelled => "trade_cancelled",
            TradeResult::Unknown => "trade_unknown",
        }
    }
}
