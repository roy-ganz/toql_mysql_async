//! A reimplementation of [Queryable](crate::mysql_async::prelude::Queryable) that allows calls on [Conn] and &mut [Conn].
use mysql_async::prelude::{FromRow, StatementLike};
use mysql_async::{BoxFuture, Conn, Params, Transaction};

pub trait Queryable {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static;

    fn exec_first<'a: 'b, 'b, T, S, P>(
        &'a mut self,
        stmt: S,
        params: P,
    ) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static;

    fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b;

    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static;
}

impl Queryable for Conn {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec(self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec_first(self, stmt, params)
    }

    fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
    {
        mysql_async::prelude::Queryable::exec_drop(self, stmt, params)
    }

    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::query_first(self, query)
    }
}

impl Queryable for &mut Conn {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec(*self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec_first(*self, stmt, params)
    }
    fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
    {
        mysql_async::prelude::Queryable::exec_drop(*self, stmt, params)
    }

    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::query_first(*self, query)
    }
}

impl Queryable for Transaction<'_> {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec(self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec_first(self, stmt, params)
    }
    fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
    {
        mysql_async::prelude::Queryable::exec_drop(self, stmt, params)
    }

    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::query_first(self, query)
    }
}

impl Queryable for &mut Transaction<'_> {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec(*self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::exec_first(*self, stmt, params)
    }
    fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
    {
        mysql_async::prelude::Queryable::exec_drop(*self, stmt, params)
    }

    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static,
    {
        mysql_async::prelude::Queryable::query_first(*self, query)
    }
}
