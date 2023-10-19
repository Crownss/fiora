use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub desc: String,
    pub tag: Vec<String>,
    pub borrowed_value: u32,
    pub createdtime: DateTime<Local>,
    pub updatedtime: DateTime<Local>,
}
