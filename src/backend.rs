
use toql::backend::Backend;
use toql::sql_builder::build_result::BuildResult;
use toql::prelude::{Cache, Context, SqlArg, Sql, AliasFormat, TableMapperRegistry, ToqlError, val, Page, log_sql, log_mut_sql, log_literal_sql};



//use mysql_async::prelude::Queryable;
use crate::queryable::Queryable;

use crate::row::Row;
use crate::error::ToqlMySqlAsyncError;
use crate::result::Result;

 use std::{sync::{RwLockWriteGuard, RwLockReadGuard}, collections::{HashMap, HashSet}};
 
use async_trait::async_trait;


pub(crate) struct MySqlAsyncBackend<'a, C>
where C: Queryable + Send
 {
    pub conn: C,
    pub(crate) context: Context,
    pub(crate) cache: &'a Cache, 
}


/// Interface for Toql functions 
#[async_trait]
impl<'a, C> Backend<Row, ToqlMySqlAsyncError> for MySqlAsyncBackend<'a, C> 
where C: Queryable + Send
{
 fn registry(&self) -> std::result::Result<RwLockReadGuard<'_, TableMapperRegistry>, ToqlError> {
     self.cache.registry.read().map_err(ToqlError::from)
     
 }
   fn registry_mut(&mut self) -> std::result::Result<RwLockWriteGuard<'_, TableMapperRegistry>, ToqlError> {
      self.cache.registry.write().map_err(ToqlError::from)
   }
   fn roles(&self) -> &HashSet<String> {
       &self.context.roles
   }
   fn alias_format(&self) -> AliasFormat {
       self.context.alias_format.clone()
   }
   fn aux_params(&self) -> &HashMap<String, SqlArg> {
       &self.context.aux_params
   }
  

   async fn select_sql(&mut self, sql:Sql) -> Result<Vec<Row> >
      {
        log_sql!(&sql);
        let Sql(sql_stmt, args) = sql;

        let args = crate::sql_arg::values_from_ref(&args);
        let rows = self.conn.exec(sql_stmt, args).await?;
       
        Ok(rows.into_iter().map(|r|Row(r)).collect::<Vec<Row>>()) // Wrap new type

   }
 
   
   // Modify result, so that page with unlimited page size can be loaded
   fn prepare_page(&self, result: &mut BuildResult, page: &Page){ 
        
        let (start, number_of_records) = match page {
            Page::Uncounted(start, records) => (start, records),
            Page::Counted(start, records) => (start, records)
        };
        result.set_modifier("SQL_CALC_FOUND_ROWS".to_string());
        result.set_extra(format!("LIMIT {}, {}", start, number_of_records));

   }
   // Load page and number of records without page limitation
   async fn select_max_page_size_sql(&mut self, _sql:Sql) -> Result<u64> {
       let sql = Sql("SELECT FOUND_ROWS()".to_string(), vec![]);
       self.select_count_sql(sql).await
   }
   // Load single value
   async fn select_count_sql(&mut self, sql:Sql) -> Result<u64> {
       log_sql!(&sql);
      let Sql(sql_stmt, args) = sql;
        let args = crate::sql_arg::values_from_ref(&args);
        let row :Option<u64>= self.conn.exec_first(sql_stmt, args).await?;
        Ok(row.unwrap_or(0))
       // Ok(row.into_iter().next().unwrap().unwrap().get(0).unwrap())  
    } 

   async fn execute_sql(&mut self, sql:Sql) -> Result<()> {
       log_mut_sql!(&sql);
        let Sql(sql_stmt, args) = sql;
        let args = crate::sql_arg::values_from_ref(&args);
        self.conn.exec_drop(sql_stmt, args).await?;
        Ok(())
   }
   ///  Execute insert statement and return new keys
   async fn insert_sql(&mut self, sql:Sql) -> Result<Vec<SqlArg>>{
       log_mut_sql!(&sql);
      let Sql(sql_stmt, args) = sql;
        let args = crate::sql_arg::values_from_ref(&args);
        self.conn.exec_drop(sql_stmt, args).await?;  
        let row_count_sql= "SELECT ROW_COUNT()";
        log_literal_sql!(&row_count_sql);
        let affected_rows :Option<u64>= self.conn.query_first(row_count_sql).await?;
        let last_insert_id = "SELECT LAST_INSERT_ID()";
        log_literal_sql!(last_insert_id);
        let last_insert_id :Option<u64>= self.conn.query_first(last_insert_id).await?;
        let affected_rows = val!(affected_rows);
        let start_id = val!(last_insert_id);
        let mut ids :Vec<SqlArg>= Vec::with_capacity(affected_rows as usize);
        
        let mut id = start_id;
        for _i in 0..affected_rows {
            ids.push(SqlArg::U64(id.into()));
            id += 1;
        }
        
        Ok(ids)
        
    }
   

}