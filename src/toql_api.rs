//! Implementation of [ToqlApi] for MySQL
//! This allows to use all Toql high level functions with this backend.

use crate::{error::ToqlMySqlAsyncError, queryable::Queryable, row::Row, MySqlAsync};
use async_trait::async_trait;
use std::borrow::{Borrow, BorrowMut};
use toql::{
    backend::{count::count, delete::delete, insert::insert, load::load, update::update},
    error::ToqlError,
    keyed::Keyed,
    page::Page,
    page_counts::PageCounts,
    prelude::{FromRow, Key},
    query::Query,
    toql_api::ToqlApi,
    toql_api::{
        count::Count, delete::Delete, fields::Fields, insert::Insert, load::Load, paths::Paths,
        update::Update,
    },
};

macro_rules! toql_api {
        ($($type:ty),+) => {
            $(
#[async_trait]
impl<'a, C> ToqlApi  for $type where C:Queryable + Send
 {

    type Row = Row;
    type Error = ToqlMySqlAsyncError;

    #[tracing::instrument(skip(self, entity, paths), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
  async fn insert_one<T>(&mut self, entity: &mut T, paths: Paths) -> Result<(), Self::Error>
    where
        T: Insert
    {
         insert::<_,_,T,_,_>(&mut self.backend, &mut [entity], paths).await
    }

    /// Insert one struct.
    ///
    /// Skip fields in struct that are auto generated with `#[toql(skip_inup)]`.
    /// Returns the last generated id.
    #[tracing::instrument(skip(self, entities, paths), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn insert_many<T, Q>(&mut self, entities: &mut [Q], paths: Paths) -> Result<(), Self::Error>
    where
        T: Insert,
        Q: BorrowMut<T> + Send, {
            insert(&mut self.backend, entities, paths).await
        }

   #[tracing::instrument(skip(self, entity, fields), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn update_one<T>(&mut self, entity: &mut T, fields: Fields) -> Result<(), Self::Error>
    where
        T: Update + Keyed,
    {
          update::<_,_,T,_,_>(&mut self.backend, &mut [entity], fields).await

    }
    #[tracing::instrument(skip(self, entities, fields), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn update_many<T, Q>(&mut self, entities: &mut [Q], fields: Fields) -> Result<(), Self::Error>
    where
        T: Update + Keyed,
        Q: BorrowMut<T> + Send + Sync,
    {
            update(&mut self.backend, entities, fields).await
    }

    /// Load a struct with dependencies for a given Toql query.
    ///
    /// Returns a struct or a [ToqlMySqlAsyncError](../toql/error/enum.ToqlMySqlAsyncError.html) if no struct was found _NotFound_ or more than one _NotUnique_.
    #[tracing::instrument(skip(self, query), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
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
    #[tracing::instrument(skip(self, query), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
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
    #[tracing::instrument(skip(self, query), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn load_page<T, B>(&mut self, query: B, page: Page) -> Result<(Vec<T>, Option<PageCounts>), Self::Error>
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
    #[tracing::instrument(skip(self, query), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn count<T, B>(&mut self, query: B) -> Result<u64, Self::Error>
        where
            T: Count,
            B: Borrow<Query<T>> + Send + Sync,
        {
            count(&mut self.backend, query).await
        }

    #[tracing::instrument(skip(self, key), fields(ty = %<<K as Key>::Entity as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn delete_one<K>(&mut self, key: K) -> Result<(), Self::Error>
    where K: Key + Send, <K as Key>::Entity: Send,  <K as Key>::Entity: Delete ,
    K : Into<Query<<K as Key>::Entity>>

    {
            let query :Query<<K as Key>::Entity>= key.into();
            delete(&mut self.backend, query).await?;
            Ok(())
    }

    #[tracing::instrument(skip(self, query), fields(ty = %<T as toql::table_mapper::mapped::Mapped>::type_name()))]
    async fn delete_many<T, B>(&mut self, query: B) -> Result<(), Self::Error>
    where T: Delete, B: Borrow<Query<T>> + Send + Sync,
    <Self as ToqlApi>::Error: From<ToqlError> {
            delete(&mut self.backend, query).await?;
             Ok(())
    }
}
            )+}}

toql_api!(MySqlAsync<'a, C>, &mut MySqlAsync<'a, C>);
