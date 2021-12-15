//! The error type.
use mysql_async::{Error, FromValueError};
use std::fmt;
use toql::{error::ToqlError, sql_builder::sql_builder_error::SqlBuilderError};

/// An error type that combines all possible errors by this library.
#[derive(Debug)]
pub enum ToqlMySqlAsyncError {
    /// Error from Toql
    ToqlError(ToqlError),
    /// Database error from the MySQL
    MySqlError(Error),
    /// Deserialization error from the MySQL
    FromValueError(FromValueError),
}

impl From<Error> for ToqlMySqlAsyncError {
    fn from(err: Error) -> ToqlMySqlAsyncError {
        ToqlMySqlAsyncError::MySqlError(err)
    }
}
impl From<FromValueError> for ToqlMySqlAsyncError {
    fn from(err: FromValueError) -> ToqlMySqlAsyncError {
        ToqlMySqlAsyncError::FromValueError(err)
    }
}
impl From<ToqlError> for ToqlMySqlAsyncError {
    fn from(err: ToqlError) -> ToqlMySqlAsyncError {
        ToqlMySqlAsyncError::ToqlError(err)
    }
}
impl From<SqlBuilderError> for ToqlMySqlAsyncError {
    fn from(err: SqlBuilderError) -> ToqlMySqlAsyncError {
        ToqlMySqlAsyncError::ToqlError(err.into())
    }
}

impl fmt::Display for ToqlMySqlAsyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToqlMySqlAsyncError::ToqlError(e) => e.fmt(f),
            ToqlMySqlAsyncError::MySqlError(e) => e.fmt(f),
            ToqlMySqlAsyncError::FromValueError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ToqlMySqlAsyncError {}
