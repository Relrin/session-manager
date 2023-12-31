use deadpool_redis::{PoolError, CreatePoolError};
use derive_more::Display;
use redis::RedisError;
use serde_json::error::Error as SerdeError;
use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Validation error for the `{0}` field: {1}", field, message)]
    ValidationError {
        field: String,
        message: String,
    },
    SerdeError(String),
    RedisError(String),
}

impl Error {
    fn code(&self) -> Code {
        match self {
            Error::ValidationError { .. } => Code::InvalidArgument,
            Error::SerdeError { .. } => Code::InvalidArgument,
            Error::RedisError(_) => Code::Internal,
        }
    }

    fn message(&self) -> String {
        self.code().description().to_string()
    }

    fn details(&self) -> ErrorDetails {
        let mut details = ErrorDetails::new();

        match self {
            Error::ValidationError { field, message } => {
                details.add_bad_request_violation(field, message);
            }
            _ => {}
        };

        details
    }
}

impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        Error::RedisError(err.to_string())
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Error::RedisError(err.to_string())
    }
}

impl From<CreatePoolError> for Error {
    fn from(err: CreatePoolError) -> Self {
        Error::RedisError(err.to_string())
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::SerdeError(err.to_string())
    }
}

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        let code = err.code();
        let message = err.message();
        let details = err.details();

        Status::with_error_details(code, message, details)
    }
}
