use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    pub order_id: String,
    pub publisher: String,
    pub isbn: String,
    pub quantity: i32,
    order_date: chrono::NaiveDate,
}
