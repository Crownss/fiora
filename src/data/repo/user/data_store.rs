use crate::common::errors::Res;
use crate::data::infra::psql::TheClient;
use crate::data::repo::user::entity::{AuthEntity, ReqGetUserBy, ResGetUserBy, UserEntity};
use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

impl super::UserDataStore {
    /*
    every little pieces business model in here
    */
    pub fn new(the_client: TheClient) -> Self {
        Self { the_client }
    }

    pub async fn create_user(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        req: &UserEntity,
    ) -> Res<()> {
        let query = "insert into users (id, auth_id, full_name, address, gender, status, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8);";
        sqlx::query(query)
            .bind(&req.id)
            .bind(&req.auth_id)
            .bind(&req.full_name)
            .bind(&req.address)
            .bind(&req.gender)
            .bind(&req.status)
            .bind(&req.created_at)
            .bind(&req.updated_at)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
    pub async fn create_auth(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        req: &AuthEntity,
    ) -> Res<Uuid> {
        let query = "insert into auth (id, username, email, password, created_at, updated_at) values ($1, $2, $3, $4, $5, $6) returning id";
        let execute_query = sqlx::query(query)
            .bind(&req.id)
            .bind(&req.username)
            .bind(&req.email)
            .bind(&req.password)
            .bind(&req.created_at)
            .bind(&req.updated_at)
            .fetch_one(&mut **tx)
            .await?;
        Ok(execute_query.try_get("id")?)
    }
    pub async fn get_user_by(&self, req: ReqGetUserBy) -> Res<ResGetUserBy> {
        let query = "select u.id, a.username, a.email, a.password, u.full_name, u.address, u.gender, u.status, u.created_at, u.updated_at from users u inner join auth a on a.id=u.auth_id where u.id!=$1 and a.username!=$2 and a.email!=$3";
        let res = sqlx::query_as::<_, ResGetUserBy>(query)
            .bind(&req.id)
            .bind(&req.username)
            .bind(&req.email)
            .fetch_one(&*self.the_client)
            .await?;
        Ok(res)
    }
}
