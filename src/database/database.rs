use anyhow::Result;
use surrealdb::Surreal;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use tracing::info;

/// Setup database connection
pub async fn setup_database(
    database_url: &str,
    database_ns: &str,
    database_db: &str,
    username: &str,
    password: &str,
) -> Result<Surreal<Any>> {
    info!("Connecting to SurrealDB");

    // open a connection
    let db = any::connect(database_url).await?;
    db.use_ns(database_ns).use_db(database_db).await?;

    db.signin(Root { username, password }).await?;

    info!("SurrealDB connection established");
    Ok(db)
}
