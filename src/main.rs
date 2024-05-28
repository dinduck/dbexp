use std::error::Error;
mod mydb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("No postgres connect info!!!");
    let _pool = sqlx::PgPool::connect(&url).await?;
    Ok(())
}
