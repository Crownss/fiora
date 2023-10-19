use chrono::{DateTime, Local};
use uuid::Uuid;
use crate::domain::book::book_model::Book;

#[derive(Debug)]
pub struct BookEntity {
    pub id: Uuid,
    pub title: String,
    pub desc: String,
    pub borrowed_value: u32,
    pub tag: Vec<String>,
    pub createdtime: DateTime<Local>,
    pub updatedtime: DateTime<Local>,
}

impl Default for BookEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            title: "".to_string(),
            desc: "".to_string(),
            borrowed_value: 0,
            tag: vec![],
            createdtime: DateTime::default(),
            updatedtime: DateTime::default(),
        }
    }
}

impl From<BookEntity> for Book{
    fn from(value: BookEntity) -> Self {
        Self{
            id: value.id,
            title: value.title,
            desc: value.desc,
            tag: value.tag,
            borrowed_value: value.borrowed_value,
            createdtime: value.createdtime,
            updatedtime: value.updatedtime,
        }
    }
}