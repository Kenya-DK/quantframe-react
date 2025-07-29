pub enum ApiResponse<T> {
    Json(T),
    String(String),
    Bytes(Vec<u8>),
}
