use chrono::{DateTime, Local};
use std::fmt::Display;
use uuid::Uuid;

use crate::common::errors::CustomError;
use crate::common::errors::Res;
use crate::data::infra::psql::TheClient;
use crate::data::repo::book::entity::{BookEntity, UpdateReq};
use crate::interactor::book::book_model::Book;

use super::entity::Req;

impl super::BookDataStore {
    pub fn new(the_client: TheClient) -> Self {
        Self { the_client }
    }
    pub async fn create_book(&self, book: &Book) -> Res<()> {
        let query = "insert into books (id, title, desc, borrowed, stock, tag, createdtime, updatedtime) values ($1, $2, $3, $4, $5, $6, $7);";
        sqlx::query(query)
            .bind(&book.id)
            .bind(&book.title)
            .bind(&book.desc)
            .bind(&book.borrowed)
            .bind(&book.stock)
            .bind(&book.tag)
            .bind(&book.createdtime)
            .bind(&book.updatedtime)
            .execute(&*self.the_client)
            .await?;
        Ok(())
    }

    pub async fn list_book(&self, mut req: Req) -> Res<(Vec<BookEntity>, i64)> {
        req.page = (req.page - 1) * req.limit;
        let query = "select id, title, desc, borrowed, stock, tag, createdtime, updatedtime from books order by createdtime desc limit $1 offset $2";
        let count = "select count(id) from books";
        let res = sqlx::query_as::<_, BookEntity>(query)
            .bind(req.limit)
            .bind(req.page)
            .fetch_all(&*self.the_client)
            .await?;
        let total: (i64,) = sqlx::query_as(count).fetch_one(&*self.the_client).await?;
        Ok((res, total.0))
    }

    pub async fn update_book(&self, req: UpdateReq) -> Res<()> {
        let query_get = "select id, title, desc, borrowed, stock, tag, createdtime, updatedtime from books where";
        Ok(())
    }
}
