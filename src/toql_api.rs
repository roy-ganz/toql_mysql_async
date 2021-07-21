
use async_trait::async_trait;

use toql::keyed::Keyed;
use toql::page::Page;

use toql::query::Query;

use toql::error::ToqlError;
use toql::toql_api::ToqlApi;
 

use core::borrow::Borrow;

use crate::error::ToqlMySqlAsyncError;
use crate::MySqlAsync;

use crate::row::Row;
use toql::prelude::{FromRow, Key, ToQuery};

use std::borrow::BorrowMut;

use toql::toql_api::{fields::Fields, paths::Paths, update::Update, insert::Insert, load::Load, count::Count, delete::Delete};
 
use toql::backend::{load::load, count::count, insert::insert, update::update, delete::delete};




use mysql_async::{prelude::Queryable, Conn};



#[async_trait]
impl<'a, C> ToqlApi  for MySqlAsync<'a, C>
where C: 'a + Queryable,
 for<'b> &'b C: std::ops::Deref<Target = Conn>
 {

    type Row = Row;
    type Error = ToqlMySqlAsyncError;

  async fn insert_one<T>(&mut self, entity: &mut T, paths: Paths) -> Result<u64, Self::Error>
    where
        T: Insert 
    {
        self.insert_many::<T, _>(&mut [entity], paths).await
    }

    /// Insert one struct.
    ///
    /// Skip fields in struct that are auto generated with `#[toql(skip_inup)]`.
    /// Returns the last generated id.
    async fn insert_many<T, Q>(&mut self, entities: &mut [Q], paths: Paths) -> Result<u64, Self::Error>
    where
        T: Insert,
        Q: BorrowMut<T> + Send, {
            insert(&mut self.backend, entities, paths).await
        }

   
    async fn update_one<T>(&mut self, entity: &mut T, fields: Fields) -> Result<(), Self::Error>
    where
        T: Update,
    {
        self.update_many::<T, _>(&mut [entity], fields).await
    }

    async fn update_many<T, Q>(&mut self, entities: &mut [Q], fields: Fields) -> Result<(), Self::Error>
    where
        T: Update,
        Q: BorrowMut<T> + Send,
    {
            update(&mut self.backend, entities, fields).await
    }

    /// Load a struct with dependencies for a given Toql query.
    ///
    /// Returns a struct or a [ToqlMySqlAsyncError](../toql/error/enum.ToqlMySqlAsyncError.html) if no struct was found _NotFound_ or more than one _NotUnique_.
    async fn load_one<T, B>(&mut self, query: B) -> Result<T, Self::Error>
    where
        T: Load<Self::Row, Self::Error>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Self::Row, Self::Error>,
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
    async fn load_many<T, B>(&mut self, query: B) -> Result<Vec<T>, Self::Error>
    where
        T: Load<Self::Row, Self::Error>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Self::Row, Self::Error>,
    {
      let res = load(&mut self.backend, query, None).await?;
      Ok(res.0)
    }

  /// Load a vector of structs with dependencies for a given Toql query.
    ///
    /// Returns a tuple with the structs and an optional tuple of count values.
    /// If `count` argument is `false`, no count queries are run and the resulting `Option<(u32,u32)>` will be `None`
    /// otherwise the count queries are run and it will be `Some((unpaged count, unfiltered count))`.
    async fn load_page<T, B>(&mut self, query: B, page: Page) -> Result<(Vec<T>, Option<(u64, u64)>), Self::Error>
    where
        T: Load<Self::Row, Self::Error>,
        B: Borrow<Query<T>> + Send + Sync,
        <T as Keyed>::Key: FromRow<Self::Row, Self::Error>,
    {
        let entities_page = load(&mut self.backend, query, Some(page)).await?;

        Ok(entities_page)
    }

    /// Counts the number of rows that match the query predicate.
    ///
    /// Returns a struct or a [ToqlMySqlAsyncError](../toql/error/enum.ToqlMySqlAsyncError.html) if no struct was found _NotFound_ or more than one _NotUnique_.
    async fn count<T, B>(&mut self, query: B) -> Result<u64, Self::Error>
        where
            T: Count,
            B: Borrow<Query<T>> + Send + Sync,
        {
            count(&mut self.backend, query).await
        }
    async fn delete_one<K, B>(&mut self, key: B) -> Result<u64, Self::Error>
    where  B: Borrow<K> + Send, K: Key + ToQuery<<K as Key>::Entity> + Send, <K as Key>::Entity: Send,  <K as Key>::Entity: Delete 
    {
            let query = key.borrow().to_query();
            delete(&mut self.backend, query).await?;
            Ok(0)
    }

    async fn delete_many<T, B>(&mut self, query: B) -> Result<u64, Self::Error>
    where T: Delete, B: Borrow<Query<T>> + Send + Sync, 
    <Self as ToqlApi>::Error: From<ToqlError> {
            delete(&mut self.backend, query).await?;
             Ok(0)
    }
}