use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth::Root;
use surrealdb::sql;
use surrealdb::{Surreal, Result};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Datastore::new("memory").await?;
    let ses = Session::for_db("my_ns", "my_db");

    // Create
    let sql = "CREATE task SET title = 'Task 01', status = 'pending'";
    let ress = ds.execute(sql, &ses, None, false).await?;
    let sql = "CREATE task SET title = 'Task 02', status = 'pending'";
    let ress = ds.execute(sql, &ses, None, false).await?;

    // Select
    let sql = "SELECT * from task";
    let ress = ds.execute(sql, &ses, None, false).await?;

    println!("{ress:?}");

    Ok(())
}
