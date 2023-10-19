use crate::common::errors::CustomError;
use std::convert::TryFrom;
use chrono::DateTime;
use serde::Deserialize;
use crate::domain::user::user_model::User;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub conf_password: String,
}

impl TryFrom<CreateUserRequest> for User{
    type Error = CustomError;
    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        Ok(
            User{
                id: Uuid::new_v4(),
                first_name: value.first_name,
                username: value.username,
                password: value.password,
                email: value.email,
                last_name: value.last_name,
                createdtime: DateTime::default(),
                updatedtime: DateTime::default(),
            }
        )
    }
}