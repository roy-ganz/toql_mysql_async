use crate::prelude::{MySqlAsync, ToqlMySqlAsyncError};
use mysql_async::prelude::*;
use mysql_async::TxOpts;
use toql::prelude::ToqlApi;
use toql::prelude::{paths, query, Cache, Toql};

#[derive(Debug, PartialEq, Eq, Clone, Toql)]
struct Payment {
    #[toql(key)]
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[tokio::test]
async fn demo() -> Result<(), ToqlMySqlAsyncError> {
    let mut payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    let database_url = "mysql://USER:PASSWORD@localhost:3306/test";

    let pool = mysql_async::Pool::new(database_url);
    let mut conn: mysql_async::Conn = pool.get_conn().await?;

    // Create temporary table
    conn.query_drop(
        r"CREATE TEMPORARY TABLE Payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )
    .await?;

    // Toql driver
    let mut cache = Cache::default();
    let mut toql = MySqlAsync::from(conn, &mut cache);

    // Save payments
    toql.insert_many(&mut payments, paths!(top)).await?;

    // Query payments
    let loaded_payments = toql.load_many(query!(Payment, "*")).await?;

    // Dropped connection will go to the pool
    drop(toql);

    // Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;

    assert_eq!(loaded_payments, payments);

    // the async fn returns Result, so
    Ok(())
}
async fn demo_transaction() -> Result<(), ToqlMySqlAsyncError> {
    let mut payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];

    let database_url = "mysql://USER:PASSWORD@localhost:3306/test";

    let pool = mysql_async::Pool::new(database_url);
    let mut conn: mysql_async::Conn = pool.get_conn().await?;

    // Create temporary table
    conn.query_drop(
        r"CREATE TEMPORARY TABLE Payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )
    .await?;

    // Toql driver
    let mut cache = Cache::default();

    let tx_opts = TxOpts::default();
    let tx = conn.start_transaction(tx_opts).await?;
    let mut toql = MySqlAsync::from(tx, &mut cache);

    // Save payments
    toql.insert_many(&mut payments, paths!(top)).await?;

    // Query payments
    let loaded_payments = toql.load_many(query!(Payment, "*")).await?;

    let _x: Option<u64> = toql.conn().query_first("SELECT 1").await?;

    // Dropped connection will go to the pool
    let tx = toql.into_conn();
    tx.rollback().await?;

    // Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;

    assert_eq!(loaded_payments, payments);

    // the async fn returns Result, so
    Ok(())
}
