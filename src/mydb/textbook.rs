#[derive(Debug, sqlx::FromRow)]
pub struct Textbook {
    pub isbn: String,
    pub publisher: String,
}
