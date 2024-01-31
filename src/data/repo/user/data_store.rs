use crate::common::errors::Res;
use crate::data::repo::user::entity::UserEntity;
use crate::interactor::user::model::User;
use crate::{common::errors::CustomError, data::infra::psql::TheClient};
use chrono::{DateTime, Local, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

use super::entity::Req;

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

    //     pub async fn get_user_by<T: Display + std::marker::Sync + tokio_postgres::types::ToSql>(
    //         &self,
    //         param: T,
    //     ) -> Res<UserEntity> {
    //         let regexingmail = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    //         let re = Regex::new(regexingmail).unwrap();
    //         let mut query = String::from("select id, bookid, firstname, lastname, email, username, createdtime, updatedtime from users u where ");
    //         match Uuid::parse_str(&param.to_string().trim()) {
    //             Ok(_) => query.push_str("u.id=$1"),
    //             Err(_) => {
    //                 if re.is_match(&param.to_string().trim()) {
    //                     query.push_str("u.email=$1")
    //                 } else {
    //                     query.push_str("u.username=$1")
    //                 }
    //             }
    //         }
    //         let do_query = self
    //             .the_client
    //             .get()
    //             .await
    //             .unwrap()
    //             .query_one(&query, &[&param])
    //             .await?;
    //         let mut res = UserEntity::default();
    //         let id: Uuid = do_query.get(0);
    //         let bookid: Uuid = do_query.get(1);
    //         let firstname: String = do_query.get(2);
    //         let lastname: String = do_query.get(3);
    //         let email: String = do_query.get(4);
    //         let username: String = do_query.get(5);
    //         let createdtime: NaiveDateTime = do_query.get(6);
    //         let updatedtime: NaiveDateTime = do_query.get(7);
    //         res = UserEntity {
    //             id,
    //             borrowed_book_id: bookid,
    //             first_name: firstname,
    //             last_name: lastname,
    //             email,
    //             username,
    //             password: "".to_string(),
    //             createdtime,
    //             updatedtime,
    //         };
    //         Ok(res)
    //     }
}
