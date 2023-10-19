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

// impl warp::reject::Reject for CustomError {}

// pub(crate) fn custom_reject(error: impl Into<CustomError>) -> Rejection {
//     warp::reject::custom(CustomError::HttpError(String::from(
//         error.into().to_string(),
//     )))
// }
