use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as sqlxError;
use thiserror::Error as ThisError;
use tracing::error;

use super::responses::DefaultResponse;

// use warp::Rejection;

pub type Res<T> = std::result::Result<T, CustomError>;

#[derive(Clone, Debug, ThisError)]
pub enum CustomError {
    #[error("{0}")]
    DatabaseError(String),
    #[error("{0}")]
    HttpError(String),
    #[error("{0}")]
    BadRequest(String),
    // #[error("{0}")]
    // InternalServerError(String),
    // #[error("An error occurred during general interaction {0}")]
    // GeneralError(String)
}

impl From<sqlxError> for CustomError {
    fn from(value: sqlxError) -> Self {
        match value.as_database_error() {
            Some(db_error) => {
                error!("error from database {:#?}", db_error.to_string());
                CustomError::DatabaseError(db_error.message().to_string())
            }
            None => {
                error!("error from database {:#?}", value);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        error!("got http error: {}", self.to_string().clone());
        let mut resp: DefaultResponse<String> = DefaultResponse {
            status: "4000".to_string(),
            message: "Invalid Request Format!".to_string(),
            data: None,
        };
        match self {
            CustomError::BadRequest(_) => HttpResponse::Ok()
                .insert_header(ContentType::json())
                .json(resp),
            _ => {
                resp.status = "5000".to_string();
                resp.message = "Something went wrong!".to_string();
                HttpResponse::Ok()
                    .insert_header(ContentType::json())
                    .json(resp)
            }
        }
    }
}
