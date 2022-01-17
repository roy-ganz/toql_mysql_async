//! The error type.
use mysql_async::{Error, FromValueError};
use toql::{error::ToqlError, sql_builder::sql_builder_error::SqlBuilderError};
use thiserror::Error;

/// An error type that combines all possible errors by this library.
#[derive(Error, Debug)]
#[error("{0}")]
pub enum ToqlMySqlAsyncError {
    /// Error from Toql
    ToqlError(#[from] ToqlError),
    /// Database error from the MySQL
    MySqlError(#[from] Error),
    /// Deserialization error from the MySQL
    FromValueError(#[from] FromValueError),
}

impl From<SqlBuilderError> for ToqlMySqlAsyncError {
    fn from(err: SqlBuilderError) -> ToqlMySqlAsyncError {
        ToqlMySqlAsyncError::ToqlError(err.into())
    }
}

