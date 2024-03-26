use anyhow::Result;

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_poll = sqlx::SqlitePool::connect(url).await?;
    Ok(connection_poll)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Read the .env file and apply it
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connecting to: {database_url}");

    // Setup the database
    let pool = get_connection_pool(&database_url).await?;
    println!("Running migrations!");
    run_migrations(pool.clone()).await?;

    Ok(())
}
