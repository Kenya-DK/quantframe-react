export enum QfSocketEvent {
  // App
  All = "*",
  OnInitialize = "App:OnInitialize",
  UpdateSettings = "App:UpdateSettings",
  UpdateAppInfo = "App:UpdateAppInfo",
  UpdateAppError = "App:UpdateAppError",

  // Warframe Market
  UpdateOrders = "WFM:UpdateOrders",
  UpdateTransaction = "WFM:UpdateTransaction",
  UpdateAuction = "WFM:UpdateAuction",

  // Chat
  UpdateChats = "WFM:UpdateChats",
  ChatReceiveMessage = "Chat:ReceiveMessage",
  ChatMessageSent = "Chat:MessageSent",

  // Stock
  UpdateStockItems = "Stock:UpdateStockItems",
  UpdateStockRivens = "Stock:UpdateStockRivens",

  // Wish List
  UpdateWishList = "WishList:Update",

  // User
  UpdateUser = "User:Update",

  // Live Trading
  UpdateLiveTradingRunningState = "LiveTrading:UpdateRunningState",
  OnLiveTradingError = "LiveTrading:OnError",
  OnLiveTradingMessage = "LiveTrading:OnMessage",

  // Notification
  OnNotificationError = "Notification:OnError",
  OnNotificationWarning = "Notification:OnWarning",
  OnNotificationSuccess = "Notification:OnSuccess",

  // Control
  OnToggleControl = "Control:OnToggleControl",
}

export enum QfSocketEventOperation {
  CREATE_OR_UPDATE = "CREATE_OR_UPDATE",
  DELETE = "DELETE",
  SET = "SET",
}
