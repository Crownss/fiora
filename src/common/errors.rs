use std::fmt::Error;
use thiserror::__private::AsDynError;
use thiserror::Error as ThisError;
use tokio_postgres::Error as psqlError;

// use warp::Rejection;

pub type Res<T> = std::result::Result<T, CustomError>;

#[derive(Clone, Debug, ThisError)]
pub enum CustomError {
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String),
    #[error("An error occurred during http interaction. {0}")]
    HttpError(String),
    #[error("An error occurred during general interaction {0}")]
    GeneralError(String)
}

impl From<psqlError> for CustomError {
    fn from(psql_error: psqlError) -> Self {
        match psql_error.as_db_error() {
            Some(db_error) => CustomError::DatabaseError(db_error.to_string()),
            None => {
                eprintln!("error {:?}", psql_error);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}

// impl From<Error> for CustomError{
//     fn from(value: Error) -> Self {
//         match value.as_dyn_error() {
//             Some(res) => CustomError::GeneralError(res.to_string()),
//             None => {
//                 eprintln!("error {:?}", value);
//                 CustomError::GeneralError(String::from("Unrecognized general error!"))
//             }
//         }
//     }
// }

// impl warp::reject::Reject for CustomError {}

// pub(crate) fn custom_reject(error: impl Into<CustomError>) -> Rejection {
//     warp::reject::custom(CustomError::HttpError(String::from(
//         error.into().to_string(),
//     )))
// }
