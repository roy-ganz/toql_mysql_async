//! A Result type with a [ToqlMySqlAsyncError] error.

use crate::error::ToqlMySqlAsyncError;

/// A result with a [ToqlMySqlAsyncError]
pub type Result<T> = std::result::Result<T, ToqlMySqlAsyncError>;
