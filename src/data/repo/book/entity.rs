use crate::interactor::book::book_model::Book;
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BookEntity {
    pub id: Uuid,
    pub title: String,
    pub desc: String,
    pub borrowed: i32,
    pub stock: i32,
    pub tag: String,
    pub createdtime: NaiveDateTime,
    pub updatedtime: NaiveDateTime,
}

pub struct Req {
    pub limit: i64,
    pub page: i64,
}

pub struct UpdateReq {
    pub id: Uuid,                   //required
    pub updatedtime: NaiveDateTime, //required
    pub title: String,
    pub desc: String,
    pub borrowed: i32,
    pub stock: i32,
    pub tag: Vec<String>,
}

impl Default for BookEntity {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            title: "".to_string(),
            desc: "".to_string(),
            borrowed: 0,
            stock: 0,
            tag: "".to_string(),
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
            borrowed: value.borrowed,
            stock: value.stock,
            createdtime: value.createdtime,
            updatedtime: value.updatedtime,
        }
    }
}
