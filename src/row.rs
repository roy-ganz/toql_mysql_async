//! Newtype for MySQL [Row](mysql_async::Row) type.
//! This allows to implement the conversion trait [FromRow]( toql::from_row::FromRow) for basic data types
//! without violating the orphan rule.

use crate::error::ToqlMySqlAsyncError;
use mysql_common::chrono::NaiveDateTime;
use toql::sql_builder::select_stream::Select;

#[derive(Debug)]
pub struct Row(pub mysql_async::Row);

macro_rules! from_row {
        ($($type:ty),+) => {
            $(
               impl toql::from_row::FromRow<Row, ToqlMySqlAsyncError> for $type {
               fn forward<'a, I>( iter: &mut I) -> Result<usize,ToqlMySqlAsyncError>
                where
                        I: Iterator<Item = &'a Select>{
                    if  iter.next().ok_or(
                            toql::error::ToqlError::DeserializeError(
                                toql::deserialize::error::DeserializeError::StreamEnd)
                    )?.is_selected() {
                        Ok(1)
                    } else {
                        Ok(0)
                    }
                }
                // Return None, if unselected or column is null
                fn from_row<'a, I>(
                        row: &Row,
                        i: &mut usize,
                        iter: &mut I,
                    ) -> std::result::Result<Option<$type>, ToqlMySqlAsyncError>
                    where
                        I: Iterator<Item = &'a Select> + Clone,
                    {
                        if iter
                       // .inspect(|v| println!("Select is {:?}", v))
                         . next().ok_or(
                            toql::error::ToqlError::DeserializeError(
                                toql::deserialize::error::DeserializeError::StreamEnd)
                         )?.is_selected() {
                            // First Option is None, if Index is out of bounds, second Option is Nullable column
                             let v :Option<Result<Option<$type>, mysql_async::FromValueError>>  = row.0.get_opt(*i);
                             let v = v.ok_or(toql::error::ToqlError::DeserializeError(
                            toql::deserialize::error::DeserializeError::StreamEnd))?;

                            let v = v.map_err(ToqlMySqlAsyncError::from)?;
                            *i += 1;
                            Ok(v)
                        } else {
                            Ok(None)
                        }
                    }
                }

            )+
        };
        }

from_row!(
    NaiveDateTime,
    String,
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    bool
);
