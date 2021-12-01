


# MySQL Async support for Toql

Add this to your `Cargo.toml`:

```toml
[dependencies]
toql = {version = "0.3", features = ["serde"]}
toql_mysql_async = "0.3"
```

And get your Toql with 

```rust
use toql_mysql_async::{prelude::MySqlAsync, mysql_async::Pool};
use toql::prelude::Cache;

let database_url = "mysql://USER:PASS@localhost:3306/DATABASE";
let pool = Pool::new(database_url);
let mut conn = pool.get_conn().await?;
let cache = Cache::new();
let mut toql = MySqlAsync::from(conn, &cache);
```

A transaction can be started from a connection:
```rust
use toql_mysql_async::mysql_async::TxOpts;

// let conn = ...
// let cache = ...

let tx_opts = TxOpts::default();
let tx = conn.start_transaction(tx_opts).await?;
let mut toql = MySqlAsync::from(tx, &cache);
 ```

## License
Toql MySqlAsync is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

