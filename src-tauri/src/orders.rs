#[tauri::command]
async fn get_orders(
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<Value, GlobleError> {
    let wfm = wfm.lock()?.clone();
}
#[tauri::command]
async fn delete_order(
    id: String,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<Value, GlobleError> {
    let wfm = wfm.lock()?.clone();
}
#[tauri::command]
async fn create_order(
    id: String,
    order_type: String,
    quantity: i64,
    price: i64,
    rank: i64,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<Value, GlobleError> {
    let wfm = wfm.lock()?.clone();
}
#[tauri::command]
async fn update_order(
    id: String,
    order_type: String,
    quantity: i64,
    price: i64,
    rank: i64,
    wfm: tauri::State<'_, Arc<Mutex<WFMClientState>>>,
) -> Result<Value, GlobleError> {
    let wfm = wfm.lock()?.clone();
}
