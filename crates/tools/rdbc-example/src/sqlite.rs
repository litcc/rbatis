

use rbdc::db::Connection;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;
use rbdc::Error;
use rbdc_pool_fast::FastPool;
use rbdc_sqlite::SqliteDriver;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = FastPool::new(ConnManager::new(
        SqliteDriver {},
        "sqlite://target/test.db",
    )?)?;
    let mut conn = pool.get().await?;
    let v = conn.get_values("select * from sqlite_master", vec![]).await?;
    println!("{}", rbs::Value::Array(v));
    Ok(())
}
