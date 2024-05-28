use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Reservation {
    pub reservation_id: String,
    pub student_id: String,
    pub isbn: String,
    pub resdate: chrono::NaiveDate,
    pub quantity: i32,
}
