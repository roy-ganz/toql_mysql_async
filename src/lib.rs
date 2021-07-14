//!
//! The Toql MySQL integration facade functions to load a struct from a MySQL database and insert, delete and update it.
//! The actual functionality is created by the Toql Derive that implements
//! the trait [Mutate](../toql/mutate/trait.Mutate.html).
//!



use crate::row::Row;

//use toql::mutate::collection_delta_sql;



use toql::keyed::Keyed;
use toql::page::Page;

use toql::query::Query;

use toql::sql_mapper_registry::SqlMapperRegistry;

use toql::error::ToqlError;
 use toql::paths::Paths;

use core::borrow::Borrow;
use toql::alias_format::AliasFormat;

use crate::error::ToqlMySqlAsyncError;
use toql::prelude::FromRow;


//use crate::row::FromResultRow;
use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    sync::RwLockReadGuard,
};

 use toql::fields::Fields;
 use toql::backend::update::{update, Update};
 use toql::backend::insert::{insert, Insert};
 use toql::backend::load::{load, Load};
 use toql::backend::count::{count, Count};
 


#[macro_use]
pub mod access;

pub mod sql_arg;

pub mod error;
pub mod result;
pub mod row;

pub mod backend;

#[cfg(test)]
mod test;

pub use mysql_async; // Reexport for derive produced code

use mysql_async::prelude::Queryable;
use mysql_async::Conn;

use crate::backend::MySqlAsyncBackend;

use toql::{prelude::{Context, Cache, SqlArg}};

use result::Result;


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

    pub async fn insert_one<T>(&mut self, entity: &mut T, paths: Paths) -> Result<u64>
    where
        T: Insert 
    {
        self.insert_many::<T, _>(&mut [entity], paths).await
    }

    /// Insert one struct.
    ///
    /// Skip fields in struct that are auto generated with `#[toql(skip_inup)]`.
    /// Returns the last generated id.
    pub async fn insert_many<T, Q>(&mut self, entities: &mut [Q], paths: Paths) -> Result<u64>
    where
        T: Insert,
        Q: BorrowMut<T>, {
            insert(&mut self.backend, entities, paths).await
        }

   
    pub async fn update_one<T>(&mut self, entity: &mut T, fields: Fields) -> Result<()>
    where
        T: Update,
    {
        self.update_many::<T, _>(&mut [entity], fields).await
    }

    pub async fn update_many<T, Q>(&mut self, entities: &mut [Q], fields: Fields) -> Result<()>
    where
        T: Update,
        Q: BorrowMut<T>,
    {
            update(&mut self.backend, entities, fields).await
    }

    /// Load a struct with dependencies for a given Toql query.
    ///
    /// Returns a struct or a [ToqlMySqlAsyncError](../toql/error/enum.ToqlMySqlAsyncError.html) if no struct was found _NotFound_ or more than one _NotUnique_.
    pub async fn load_one<T, B>(&mut self, query: B) -> Result<T>
    where
        T: Load<Row, ToqlMySqlAsyncError>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Row, ToqlMySqlAsyncError>,
    {
        let (mut e, _) = load(&mut self.backend, query, Some(Page::Uncounted(0, 2))).await?;
        match e.len() {
            0 => Err(ToqlError::NotFound.into()),
            1 => Ok(e.pop().unwrap()),
            _ => Err(ToqlError::NotUnique.into()),
        }
    }

    /// Load a vector of structs with dependencies for a given Toql query.
    ///
    /// Returns a tuple with the structs and an optional tuple of count values.
    /// If `count` argument is `false`, no count queries are run and the resulting `Option<(u32,u32)>` will be `None`
    /// otherwise the count queries are run and it will be `Some((total count, filtered count))`.
    pub async fn load_many<T, B>(&mut self, query: B) -> Result<Vec<T>>
    where
        T: Load<Row, ToqlMySqlAsyncError>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Row, ToqlMySqlAsyncError>,
    {
      let res = load(&mut self.backend, query, None).await?;
      Ok(res.0)
    }

  /// Load a vector of structs with dependencies for a given Toql query.
    ///
    /// Returns a tuple with the structs and an optional tuple of count values.
    /// If `count` argument is `false`, no count queries are run and the resulting `Option<(u32,u32)>` will be `None`
    /// otherwise the count queries are run and it will be `Some((unpaged count, unfiltered count))`.
    pub async fn load_page<T, B>(&mut self, query: B, page: Page) -> Result<(Vec<T>, Option<(u64, u64)>)>
    where
        T: Load<Row, ToqlMySqlAsyncError>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Row, ToqlMySqlAsyncError>,
    {
        let entities_page = load(&mut self.backend, query, Some(page)).await?;

        Ok(entities_page)
    }

    /// Counts the number of rows that match the query predicate.
    ///
    /// Returns a struct or a [ToqlMySqlAsyncError](../toql/error/enum.ToqlMySqlAsyncError.html) if no struct was found _NotFound_ or more than one _NotUnique_.
    pub async fn count<T, B>(&mut self, query: B) -> Result<u64>
        where
            T: Count,
            B: Borrow<Query<T>> + Send + Sync,
        {
            count(&mut self.backend, query).await
        }
}
