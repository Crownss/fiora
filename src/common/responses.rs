use serde::Serialize;
#[derive(Serialize)]
pub struct DefaultResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ResWCount<T> {
    pub total: u64,
    pub items: T,
}
