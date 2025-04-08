#[derive(PartialEq, Debug, Clone)]
pub enum UIEvent {
    OnInitialize,
    UpdateSettings,
    UpdateAppInfo,
    UpdateOrders,
    UpdateTransaction,
    UpdateAuction,
    UpdateChats,
    UpdateStockItems,
    UpdateStockRivens,
    UpdateWishList,
    UpdateUser,
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
            UIEvent::UpdateTransaction => "WFM:UpdateTransaction",
            UIEvent::UpdateAuction => "WFM:UpdateAuction",
            UIEvent::UpdateChats => "WFM:UpdateChats",
            UIEvent::UpdateStockItems => "Stock:UpdateStockItems",
            UIEvent::UpdateStockRivens => "Stock:UpdateStockRivens",
            UIEvent::UpdateWishList => "WishList:Update",
            UIEvent::UpdateUser => "User:Update",
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
