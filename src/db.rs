use dotenv::dotenv;

pub async fn connect() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("No postgres connect info!!!");
    let pool = sqlx::PgPool::connect(&url).await?;
    Ok(pool)
}
