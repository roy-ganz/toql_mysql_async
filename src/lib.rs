//! # MySQL Async support for Toql
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! toql = {version = "0.3", features = ["serde"]}
//! toql_mysql_async = "0.3"
//! ```
//!
//! And get your Toql with
//!
//! ```rust
//! use toql_mysql_async::{prelude::MySqlAsync, mysql_async::Pool};
//! use toql::prelude::Cache;
//!
//! let database_url = "mysql://USER:PASS@localhost:3306/DATABASE";
//! let pool = Pool::new(database_url);
//! let mut conn = pool.get_conn().await?;
//! let cache = Cache::new();
//! let mut toql = MySqlAsync::from(conn, &cache);
//! ```
//!
//! A transaction can be started from a connection:
//! ```rust
//! use toql_mysql_async::mysql_async::TxOpts;
//!
//! // let conn = ...
//! // let cache = ...
//!
//! let tx_opts = TxOpts::default();
//! let tx = conn.start_transaction(tx_opts).await?;
//! let mut toql = MySqlAsync::from(tx, &cache);
//! ```
//!
//! ## License
//! Toql MySqlAsync is distributed under the terms of both the MIT license and the
//! Apache License (Version 2.0).

use toql::table_mapper_registry::TableMapperRegistry;

use toql::error::ToqlError;

use toql::alias_format::AliasFormat;

use std::{
    collections::{HashMap, HashSet},
    sync::RwLockReadGuard,
};

#[macro_use]
pub mod access;

pub mod sql_arg;

pub mod error;
pub mod result;
pub mod row;

pub mod backend;

pub mod prelude;
pub mod toql_api;

pub mod queryable;

// Reexport for derive produced code
pub use mysql_async;

#[cfg(test)]
mod test;

use crate::backend::MySqlAsyncBackend;
use toql::prelude::{Cache, Context, SqlArg};
//use mysql_async::prelude::Queryable;
use crate::queryable::Queryable;

pub struct MySqlAsync<'a, C>
where
    C: Queryable + Send,
{
    backend: MySqlAsyncBackend<'a, C>,
}

/// Public API
impl<'a, C> MySqlAsync<'a, C>
where
    C: Queryable + Send,
{
    /// Create connection wrapper from MySqlAsync connection or transaction.
    ///
    /// Use the connection wrapper to access all Toql functionality.
    pub fn from(conn: C, cache: &'a Cache) -> MySqlAsync<'a, C> {
        Self::with_context(conn, cache, Context::default())
    }

    pub fn conn(&mut self) -> &mut C {
        &mut self.backend.conn
    }

    pub fn into_conn(self) -> C {
        self.backend.conn
    }

    pub fn with_context(conn: C, cache: &'a Cache, context: Context) -> MySqlAsync<'a, C> {
        MySqlAsync {
            backend: MySqlAsyncBackend {
                conn,
                cache,
                context,
            },
        }
    }

    pub fn set_roles(&mut self, roles: HashSet<String>) -> &mut Self {
        self.backend.context.roles = roles;
        self
    }

    pub fn registry(
        &self,
    ) -> std::result::Result<RwLockReadGuard<'_, TableMapperRegistry>, ToqlError> {
        self.backend.cache.registry.read().map_err(ToqlError::from)
    }
    pub fn roles(&self) -> &HashSet<String> {
        &self.backend.context.roles
    }

    pub fn alias_format(&self) -> AliasFormat {
        self.backend.context.alias_format.to_owned()
    }

    pub fn aux_params(&self) -> &HashMap<String, SqlArg> {
        &self.backend.context.aux_params
    }
    pub fn set_aux_param(&mut self, name: String, value: SqlArg) {
        &self.backend.context.aux_params.insert(name, value);
    }
}
