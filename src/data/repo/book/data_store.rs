use chrono::{DateTime, Local};
use std::fmt::Display;
use uuid::Uuid;

use crate::common::errors::CustomError;
use crate::common::errors::Res;
use crate::data::infra::psql::TheClient;
use crate::data::repo::book::entity::BookEntity;
use crate::interactor::book::book_model::Book;

use super::entity::Req;

impl super::BookDataStore {
    pub fn new(the_client: TheClient) -> Self {
        Self { the_client }
    }
    pub async fn create_book(&self, book: &Book) -> Res<()> {
        let query = "insert into books (id, title, desc, value, tag, createdtime, updatedtime) values ($1, $2, $3, $4, $5, $6, $7);";
        sqlx::query(query)
            .bind(&book.id)
            .bind(&book.title)
            .bind(&book.desc)
            .bind(&book.borrowed_value)
            .bind(&book.tag)
            .bind(&book.createdtime)
            .bind(&book.updatedtime)
            .execute(&*self.the_client)
            .await?;
        Ok(())
    }

    pub async fn list_book(&self, mut req: Req) -> Res<(Vec<BookEntity>, i64)> {
        req.page = (req.page - 1) * req.limit;
        let query = "select id, title, desc, value, tag, createdtime, updatedtime from books order by createdtime desc limit $1 offset $2";
        let count = "select count(id) from books";
        let res = sqlx::query_as::<_, BookEntity>(query)
            .bind(req.limit)
            .bind(req.page)
            .fetch_all(&*self.the_client)
            .await?;
        let total: (i64,) = sqlx::query_as(count).fetch_one(&*self.the_client).await?;
        Ok((res, total.0))
    }

    // pub async fn get_book_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql>(
    //     &self,
    //     param: T,
    // ) -> Res<BookEntity> {
    //     let mut query = String::from(
    //         "select id, title, desc, value, tag, createdtime, updatedtime from books b where ",
    //     );
    //     match Uuid::parse_str(&param.to_string().trim()) {
    //         Ok(res) => query.push_str("b.id=$1"),
    //         Err(_) => query.push_str("b.title=$1"),
    //     };
    //     let mut res = BookEntity::default();
    //     let do_query = self
    //         .the_client
    //         .get()
    //         .await
    //         .unwrap()
    //         .query_one(&query, &[&param])
    //         .await?;
    //     let id: Uuid = do_query.get(0);
    //     let title: String = do_query.get(1);
    //     let desc: String = do_query.get(2);
    //     let value: u32 = do_query.get(3);
    //     let tag: Vec<String> = do_query.get(4);
    //     let createdtime: DateTime<Local> = do_query.get(5);
    //     let updatedtime: DateTime<Local> = do_query.get(6);
    //     res = BookEntity {
    //         id,
    //         title,
    //         desc,
    //         borrowed_value: value,
    //         tag,
    //         createdtime,
    //         updatedtime,
    //     };
    //     Ok(res)
    // }
}
