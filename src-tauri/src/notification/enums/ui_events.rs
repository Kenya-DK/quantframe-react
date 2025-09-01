use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum UIEvent {
    OnStartingUp,
    UpdateUser,
    // WebSocket use if the WebSocket is connected or not
    OnError,
    OnBanned,
    UpdateLiveScraperRunningState,
    SendLiveScraperMessage,
    RefreshStockItems,
    RefreshStockRivens,
    RefreshWishListItems,
    RefreshWfmOrders,
    OnDeleteWfmOrders,
    OnDeleteWfmAuctions,
    RefreshWfmAuctions,
}
impl UIEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIEvent::OnStartingUp => "App:StartingUp",
            UIEvent::UpdateUser => "User:Update",
            UIEvent::OnError => "App:Error",
            UIEvent::OnBanned => "User:Banned",
            UIEvent::UpdateLiveScraperRunningState => "LiveScraper:UpdateRunningState",
            UIEvent::SendLiveScraperMessage => "LiveScraper:OnMessage",
            UIEvent::RefreshStockItems => "LiveScraper:RefreshStockItems",
            UIEvent::RefreshStockRivens => "LiveScraper:RefreshStockRivens",
            UIEvent::RefreshWishListItems => "LiveScraper:RefreshWishListItems",
            UIEvent::RefreshWfmOrders => "LiveScraper:RefreshWfmOrders",
            UIEvent::OnDeleteWfmOrders => "Wfm:OnDeleteOrders",
            UIEvent::OnDeleteWfmAuctions => "Wfm:OnDeleteAuctions",
            UIEvent::RefreshWfmAuctions => "Wfm:RefreshAuctions",
        }
    }
}

impl Display for UIEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UIOperationEvent {
    CreateOrUpdate,
    Delete,
    Set,
}
impl UIOperationEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIOperationEvent::CreateOrUpdate => "CREATE_OR_UPDATE",
            UIOperationEvent::Delete => "DELETE",
            UIOperationEvent::Set => "SET",
        }
    }
}
