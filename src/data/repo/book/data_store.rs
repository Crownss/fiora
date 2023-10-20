use std::fmt::Display;
use chrono::{DateTime, Local};
use uuid::Uuid;
use bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};

use crate::common::errors::CustomError;
use crate::common::errors::Res;
use crate::data::repo::book::entity::BookEntity;
use crate::domain::book::book_model::Book;

impl super::BookDataStore {
    pub fn new(the_client: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { the_client }
    }
    pub async fn create_book(&self, book: &Book) -> Res<()> {
        let query = "insert into books (id, title, desc, value, tag, createdtime, updatedtime) values ($1, $2, $3, $4, $5, $6, $7);";
        self.the_client.get().await.unwrap()
            .execute(
                query,
                &[
                    &book.id,
                    &book.title,
                    &book.desc,
                    &book.borrowed_value,
                    &book.tag,
                    &book.createdtime,
                    &book.updatedtime,
                ],
            )
            .await
            .map_err(CustomError::from)?;
        Ok(())
    }

    pub async fn list_book(&self) -> Res<Vec<BookEntity>> {
        let query = "select id, title, desc, value, tag, createdtime, updatedtime from books";
        let mut res = Vec::new();
        let do_query = self.the_client.get().await.unwrap().query(query, &[]).await?;
        for me in do_query {
            let id: Uuid = me.get(0);
            let title: String = me.get(1);
            let desc: String = me.get(2);
            let value: u32 = me.get(3);
            let tag: Vec<String> = me.get(4);
            let createdtime: DateTime<Local> = me.get(5);
            let updatedtime: DateTime<Local> = me.get(6);
            let temp = BookEntity {
                id,
                title,
                desc,
                borrowed_value: value,
                tag,
                createdtime,
                updatedtime,
            };
            res.push(temp)
        };
        Ok(res)
    }

    pub async fn get_book_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql>(&self, param: T) -> Res<BookEntity> {
        let mut query = String::from("select id, title, desc, value, tag, createdtime, updatedtime from books b where ");
        match Uuid::parse_str(&param.to_string().trim()) {
            Ok(res) => { query.push_str("b.id=$1") }
            Err(_) => { query.push_str("b.title=$1") }
        };
        let mut res = BookEntity::default();
        let do_query = self.the_client.get().await.unwrap().query_one(&query, &[&param]).await?;
        let id: Uuid = do_query.get(0);
        let title: String = do_query.get(1);
        let desc: String = do_query.get(2);
        let value: u32 = do_query.get(3);
        let tag: Vec<String> = do_query.get(4);
        let createdtime: DateTime<Local> = do_query.get(5);
        let updatedtime: DateTime<Local> = do_query.get(6);
        res = BookEntity {
            id,
            title,
            desc,
            borrowed_value: value,
            tag,
            createdtime,
            updatedtime,
        };
        Ok(res)
    }
}
