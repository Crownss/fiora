use serde::Serialize;
#[derive(Serialize)]
pub struct DefaultResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Default for DefaultResponse<T> {
    fn default() -> Self {
        Self {
            status: "ERROR".to_string(),
            message: "Something wen't wrong!".to_string(),
            data: None,
        }
    }
}

#[derive(Serialize)]
pub struct ResWCount<T> {
    pub total: u64,
    pub items: T,
}
