#[derive(Debug)]
pub enum TradeResult {
    Success,
    Failed,
    Cancelled,
    Unknown,
}
