use mysql_async::prelude::{StatementLike, FromRow, Queryable};
use mysql_async::{BoxFuture, Params, Conn, Transaction};




pub trait Executable {

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

impl Executable for Conn {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        
        Queryable::exec(self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(
        &'a mut self,
        stmt: S,
        params: P,
    ) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        Queryable::exec_first(self, stmt, params)
    }
    
      fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b {
            Queryable::exec_drop(self, stmt, params)
        }
   
    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static{
        Queryable::query_first(self, query)
        }
}


impl Executable for &mut Conn {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        
        Queryable::exec(*self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(
        &'a mut self,
        stmt: S,
        params: P,
    ) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        Queryable::exec_first(*self, stmt, params)
    }
      fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b {
            Queryable::exec_drop(*self, stmt, params)
        }
   
    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static{
        Queryable::query_first(*self, query)
        }
}


impl Executable for Transaction<'_> {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        
        Queryable::exec(self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(
        &'a mut self,
        stmt: S,
        params: P,
    ) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        Queryable::exec_first(self, stmt, params)
    }
     fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b {
            Queryable::exec_drop(self, stmt, params)
        }
   
    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static{
        Queryable::query_first(self, query)
        }
}


impl Executable for &mut Transaction<'_> {
    fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, Vec<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        
        Queryable::exec(*self, stmt, params)
    }
    fn exec_first<'a: 'b, 'b, T, S, P>(
        &'a mut self,
        stmt: S,
        params: P,
    ) -> BoxFuture<'b, Option<T>>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b,
        T: FromRow + Send + 'static {
        Queryable::exec_first(*self, stmt, params)
    }
     fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> BoxFuture<'b, ()>
    where
        S: StatementLike + 'b,
        P: Into<Params> + Send + 'b {
            Queryable::exec_drop(*self, stmt, params)
        }
   
    fn query_first<'a, T, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Option<T>>
    where
        Q: AsRef<str> + Send + Sync + 'a,
        T: FromRow + Send + 'static{
        Queryable::query_first(*self, query)
        }
}