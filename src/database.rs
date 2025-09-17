use anyhow::Result;
use surrealdb::Surreal;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use tracing::info;

/// Setup database connection
pub async fn setup_database(
    database_url: &str,
    username: &str,
    password: &str,
) -> Result<Surreal<Any>> {
    info!("Connecting to SurrealDB...");

    // Open a connection
    let db = any::connect(database_url).await?;

    // Select namespace and database
    db.use_ns("urlshortener").use_db("urlshortener").await?;

    // Authenticate
    db.signin(Root { username, password }).await?;

    info!("SurrealDB connection established");
    Ok(db)
}
