use crate::interactor::book::book_model::Book;
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BookEntity {
    pub id: Uuid,
    pub title: String,
    pub desc: String,
    pub borrowed_value: i32,
    pub tag: Vec<String>,
    pub createdtime: NaiveDateTime,
    pub updatedtime: NaiveDateTime,
}

pub struct Req {
    pub limit: i64,
    pub page: i64,
}

impl Default for BookEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            title: "".to_string(),
            desc: "".to_string(),
            borrowed_value: 0,
            tag: vec![],
            createdtime: NaiveDateTime::default(),
            updatedtime: NaiveDateTime::default(),
        }
    }
}

impl From<BookEntity> for Book {
    fn from(value: BookEntity) -> Self {
        Self {
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
