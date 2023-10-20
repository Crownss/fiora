use crate::common::errors::CustomError;
use std::convert::TryFrom;
use chrono::{DateTime, NaiveDateTime};
use serde::Deserialize;
use crate::interactor::user::user_model::User;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub confPassword: String,
}

impl TryFrom<CreateUserRequest> for User{
    type Error = CustomError;
    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        Ok(
            User{
                id: Uuid::nil(),
                borrowedBookId: Uuid::nil(),
                firstName: value.firstName,
                lastName: value.lastName,
                email: value.email,
                username: value.username,
                password: value.password,
                confPassword: value.confPassword,
                createdTime: NaiveDateTime::default(),
                updatedTime: NaiveDateTime::default(),
            }
        )
    }
}