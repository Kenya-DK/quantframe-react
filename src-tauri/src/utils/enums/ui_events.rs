#[derive(PartialEq, Debug, Clone)]
pub enum UIEvent {
    OnInitialize,
    UpdateSettings,
    UpdateAppInfo,
    UpdateOrders,
    RefreshTransactions,
    UpdateAuction,
    UpdateChats,
    ChatMessageSent,
    RefreshStockItems,
    RefreshStockRivens,
    RefreshWishListItems,
    UpdateUser,
    UpdateUserStatus,
    UpdateAlerts,
    UpdateLiveTradingRunningState,
    OnLiveTradingError,
    OnLiveTradingMessage,
    // Notification
    OnNotificationWarning,
    OnNotificationSuccess,
    OnToggleControl,
    ReceiveMessage,
}
impl UIEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIEvent::OnInitialize => "App:OnInitialize",
            UIEvent::UpdateSettings => "App:UpdateSettings",
            UIEvent::UpdateAppInfo => "App:UpdateAppInfo",
            UIEvent::UpdateOrders => "WFM:UpdateOrders",
            UIEvent::RefreshTransactions => "WFM:RefreshTransactions",
            UIEvent::UpdateAuction => "WFM:UpdateAuction",
            UIEvent::UpdateChats => "WFM:UpdateChats",
            UIEvent::ChatMessageSent => "Chat:MessageSent",
            UIEvent::RefreshStockItems => "Stock:RefreshStockItems",
            UIEvent::RefreshStockRivens => "Stock:RefreshStockRivens",
            UIEvent::RefreshWishListItems => "Stock:RefreshWishListItems",
            UIEvent::UpdateUser => "User:Update",
            UIEvent::UpdateUserStatus => "User:UpdateStatus",
            UIEvent::UpdateLiveTradingRunningState => "LiveTrading:UpdateRunningState",
            UIEvent::OnLiveTradingError => "LiveTrading:OnError",
            UIEvent::OnLiveTradingMessage => "LiveTrading:OnMessage",
            UIEvent::OnNotificationWarning => "Notification:OnWarning",
            UIEvent::OnNotificationSuccess => "Notification:OnSuccess",
            UIEvent::OnToggleControl => "Control:OnToggleControl",
            UIEvent::UpdateAlerts => "Alert:Update",
            UIEvent::ReceiveMessage => "Chat:ReceiveMessage",
        }
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
