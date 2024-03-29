use chrono::{NaiveDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub desc: String,
    pub tag: Vec<String>,
    pub borrowed_value: i32,
    pub createdtime: NaiveDateTime,
    pub updatedtime: NaiveDateTime,
}
