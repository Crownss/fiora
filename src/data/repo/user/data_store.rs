use crate::common::errors::Res;
use crate::data::repo::user::entity::UserEntity;
use crate::interactor::user::model::User;
use crate::{common::errors::CustomError, data::infra::psql::TheClient};
use chrono::{DateTime, Local, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

use super::entity::{Req, ReqFilter};

impl super::UserDataStore {
    pub fn new(the_client: TheClient) -> Self {
        Self { the_client }
    }

    pub async fn create_user(&self, user: &User) -> Res<()> {
        let query = "insert into users (id, bookid, firstname, lastname, email, username, password, createdtime, updatedtime) values ($1, $2, $3, $4, $5, $6, $7, $8, $9);";
        sqlx::query(query)
            .bind(&user.id)
            .bind(&user.borrowedBookId)
            .bind(&user.firstName)
            .bind(&user.lastName)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.createdTime)
            .bind(&user.updatedTime)
            .execute(&self.the_client)
            .await?;
        Ok(())
    }

    pub async fn list_user(&self, mut req: Req) -> Res<(Vec<UserEntity>, i64)> {
        req.page = (req.page - 1) * req.limit;
        let query =
            "select id, bookid as borrowed_book_id, firstname as first_name, lastname as last_name, email, username, createdtime, updatedtime from users order by createdtime desc limit $1 offset $2";
        let count = "select count(id) from users";
        let res = sqlx::query_as::<_, UserEntity>(query)
            .bind(req.limit)
            .bind(req.page)
            .fetch_all(&self.the_client)
            .await?;
        let total: (i64,) = sqlx::query_as(count).fetch_one(&self.the_client).await?;
        Ok((res, total.0))
    }

    pub async fn get_user_by(&self, req: ReqFilter) -> Res<UserEntity> {
        let mut query = String::from("select id, bookid as borrowed_book_id, firstname as first_name, lastname as last_name, email, username, createdtime, updatedtime from users where email!=$1 and firstname!=$2 and last_name!=$3 and username!=$4 and borrowed_book_id!=$5");
        if req.email != "" {
            query = query.replace("email!=", "email=");
        }
        if req.first_name != "" {
            query = query.replace("firstname!=", "firstname=");
        }
        if req.last_name != "" {
            query = query.replace("lastname!=", "lastname=");
        }
        if req.username != "" {
            query = query.replace("username!=", "username=");
        }
        if req.borrowed_book_id != Uuid::nil() {
            query = query.replace("borrowed_book_id!=", "borrowed_book_id=");
        }
        let res = sqlx::query_as::<_, UserEntity>(query.as_str())
            .bind(req.email)
            .bind(req.first_name)
            .bind(req.last_name)
            .bind(req.username)
            .bind(req.borrowed_book_id)
            .fetch_one(&self.the_client)
            .await?;
        Ok(res)
    }
}
