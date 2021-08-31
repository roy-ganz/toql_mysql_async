//!
//! The Toql MySQL integration facade functions to load a struct from a MySQL database and insert, delete and update it.
//! The actual functionality is created by the Toql Derive that implements
//! the trait [Mutate](../toql/mutate/trait.Mutate.html).
//!




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

pub mod toql_api;
pub mod prelude;

pub mod queryable;

// Reexport for derive produced code
pub use mysql_async; 

#[cfg(test)]
mod test;


use crate::backend::MySqlAsyncBackend;
use toql::prelude::{Context, Cache, SqlArg};
//use mysql_async::prelude::Queryable;
use crate::queryable::Queryable;
use std::ops::Deref;
use crate::result::Result;

pub struct MySqlAsync<'a, C> where C: Queryable + Send
{
    backend: MySqlAsyncBackend<'a, C>,
   
}
   

/// Public API 
impl<'a, C> MySqlAsync<'a, C> where C: Queryable + Send
{
     /// Create connection wrapper from MySqlAsync connection or transaction.
    ///
    /// Use the connection wrapper to access all Toql functionality.
    pub fn from(conn: C, cache: &'a Cache) -> MySqlAsync<'a, C> {
        
        Self::with_context(conn, cache,Context::default())
    }

        pub fn conn(&mut self) -> &mut C {
            &mut self.backend.conn
        }


        pub fn into_conn(self) -> C {
            self.backend.conn
        }


   
     pub fn with_context(conn: C, cache: &'a Cache, context: Context) -> MySqlAsync<'a, C> {
         MySqlAsync{
            backend: MySqlAsyncBackend {
                    conn,
                    cache,
                    context
                },
            }
    }

   
    pub fn set_roles(&mut self, roles: HashSet<String>) -> &mut Self {
        self.backend.context.roles = roles;
        self
    }
/* 
    pub fn conn(&mut self) -> &Conn {
       &self.backend.conn
    }

    pub fn conn_mut(&mut self) -> &mut Conn {
       &mut self.backend.conn
    } */

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