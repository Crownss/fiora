use std::fmt::Display;
use std::sync::Arc;

use chrono::{DateTime, Local};
use regex::Regex;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::common::errors::CustomError;
use crate::common::errors::Res;
use crate::data::repo::user::entity::UserEntity;
use crate::domain::user::user_model::User;

impl super::UserDataStore {
    pub fn new(the_client: Arc<Client>) -> Self {
        Self { the_client }
    }

    pub async fn create_user(&self, user: &User) -> Res<()> {
        let query = "insert into users (id, bookid, firstname, lastname, email, username, password, createdtime, updatedtime) values ($1, $2, $3, $4, $5, $6, $7, $8);";
        self.the_client
            .execute(
                query,
                &[
                    &user.id,
                    &user.first_name,
                    &user.last_name,
                    &user.email,
                    &user.username,
                    &user.password,
                    &user.createdtime,
                    &user.updatedtime,
                ],
            )
            .await
            .map_err(CustomError::from)?;
        Ok(())
    }

    pub async fn list_user(&self) -> Res<Vec<UserEntity>> {
        let query =
            "select id, bookid, firstname, lastname, email, username, createdtime, updatedtime from users";
        let do_query = self.the_client.query(query, &[]).await?;
        let mut res = Vec::new();
        for me in do_query {
            let id: Uuid = me.get(0);
            let bookid: Uuid = me.get(1);
            let firstname: String = me.get(2);
            let lastname: String = me.get(3);
            let email: String = me.get(4);
            let username: String = me.get(5);
            let createdtime: DateTime<Local> = me.get(6);
            let updatedtime: DateTime<Local> = me.get(7);
            let temp = UserEntity {
                id: id,
                borrowed_book_id: bookid,
                first_name: firstname,
                last_name: lastname,
                email: email,
                username: username,
                password: "".to_string(),
                createdtime: createdtime,
                updatedtime: updatedtime,
            };
            res.push(temp)
        };
        Ok(res)
    }

    pub async fn get_user_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql>(&self, param: T) -> Res<UserEntity> {
        let regexingmail = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
        let re = Regex::new(regexingmail).unwrap();
        let mut query = String::from("select id, bookid, firstname, lastname, email, username, createdtime, updatedtime from users u where ");
        match Uuid::parse_str(&param.to_string().trim()) {
            Ok(res) => { query.push_str("u.id=$1") }
            Err(_) => {
                if re.is_match(&param.to_string().trim()) {
                    query.push_str("u.email=$1")
                } else {
                    query.push_str("u.username=$1")
                }
            }
        }
        let do_query = self.the_client.query_one(&query, &[&param]).await?;
        let mut res = UserEntity::default();
        let id: Uuid = do_query.get(0);
        let bookid: Uuid = do_query.get(1);
        let firstname: String = do_query.get(2);
        let lastname: String = do_query.get(3);
        let email: String = do_query.get(4);
        let username: String = do_query.get(5);
        let createdtime: DateTime<Local> = do_query.get(6);
        let updatedtime: DateTime<Local> = do_query.get(7);
        res = UserEntity {
            id: id,
            borrowed_book_id: bookid,
            first_name: firstname,
            last_name: lastname,
            email: email,
            username: username,
            password: "".to_string(),
            createdtime: createdtime,
            updatedtime: updatedtime,
        };
        Ok(res)
    }
}