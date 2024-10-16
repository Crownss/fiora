use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ReqRegister {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmPassword: String,
    pub fullName: String,
    pub address: String,
    pub gender: String,
}
#[derive(Debug, Deserialize)]
pub struct ReqLogin {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct RespLogin {
    pub id: Uuid,
    pub fullName: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct ClaimJwt {
    pub id: Uuid,
    pub fullName: String,
    pub exp: i64,
}
