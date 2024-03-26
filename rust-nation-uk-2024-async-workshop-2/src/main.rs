use anyhow::Result;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_poll = sqlx::SqlitePool::connect(url).await?;
    Ok(connection_poll)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

async fn get_blog_posts(pool: sqlx::SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
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

    println!("{:?}", get_blog_posts(pool).await?);

    Ok(())
}
