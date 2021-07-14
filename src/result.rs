use crate::error::ToqlMySqlAsyncError;

/// A result with a [`ToqlError`](enum.ToqlError.html)
pub type Result<T> = std::result::Result<T, ToqlMySqlAsyncError>;