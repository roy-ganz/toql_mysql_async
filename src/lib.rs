//!
//! The Toql MySQL integration facade functions to load a struct from a MySQL database and insert, delete and update it.
//! The actual functionality is created by the Toql Derive that implements
//! the trait [Mutate](../toql/mutate/trait.Mutate.html).
//!




use toql::sql_mapper_registry::SqlMapperRegistry;

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

// Reexport for derive produced code
pub use mysql_async; 

#[cfg(test)]
mod test;


use crate::backend::MySqlAsyncBackend;
use toql::prelude::{Context, Cache, SqlArg};
use mysql_async::{Conn, prelude::Queryable};

pub struct MySqlAsync<'a, C>
where  C: 'a + Queryable,
for<'b> &'b C: std::ops::Deref<Target = Conn>
{
    backend: MySqlAsyncBackend<'a, C>
}
   

/// Public API 
impl<'a, C> MySqlAsync<'a, C> where 
 C: 'a + Queryable ,
 for<'b> &'b C: std::ops::Deref<Target = Conn>{
    /// Create connection wrapper from MySqlAsync connection or transaction.
    ///
    /// Use the connection wrapper to access all Toql functionality.
    pub fn from(conn: C, cache: &'a mut Cache) -> MySqlAsync<'a, C> {
        
        Self::with_context(conn, cache,Context::default())
    }

     pub fn with_context(conn: C, cache: &'a mut Cache, context: Context) -> MySqlAsync<'a, C> {
         MySqlAsync{
            backend: MySqlAsyncBackend {
                    conn,
                    cache,
                    context
                }
            }
    }

   
    pub fn set_roles(&mut self, roles: HashSet<String>) -> &mut Self {
        self.backend.context.roles = roles;
        self
    }

    pub fn conn(&mut self) -> &'_ mut C {
       &mut self.backend.conn
    }

    pub fn registry(
        &self,
    ) -> std::result::Result<RwLockReadGuard<'_, SqlMapperRegistry>, ToqlError> {
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