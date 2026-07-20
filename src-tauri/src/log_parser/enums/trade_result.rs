#[derive(Debug, PartialEq, Eq)]
pub enum TradeResult {
    Success,
    Failed,
    Cancelled,
    OnTradeAcceptedFailed,
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
            TradeResult::OnTradeAcceptedFailed => "OnTradeAcceptedFailed",
            TradeResult::Unknown => "Unknown",
        }
    }
    pub fn metric_name(&self) -> &'static str {
        match self {
            TradeResult::Success => "trade_accepted",
            TradeResult::Failed => "trade_failed",
            TradeResult::Cancelled => "trade_cancelled",
            TradeResult::OnTradeAcceptedFailed => "trade_accepted_failed",
            TradeResult::Unknown => "trade_unknown",
        }
    }
}
