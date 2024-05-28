#[derive(Debug, sqlx::FromRow)]
pub struct Student {
    pub student_id: i32,
    pub student_name: String,
    pub email: String,
}
