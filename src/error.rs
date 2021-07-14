
use mysql_async::{Error, FromValueError};
use toql::{error::ToqlError, sql_builder::sql_builder_error::SqlBuilderError};

#[derive(Debug)]
pub enum ToqlMySqlAsyncError {
    ToqlError(ToqlError),
    MySqlError(Error),
    FromValueError(FromValueError)
    
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


