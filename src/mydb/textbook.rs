use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Textbook {
    pub isbn: String,
    pub publisher: String,
}

pub async fn read_textbooks(
    conn: &sqlx::PgPool,
) -> Result<Vec<Textbook>, Box<dyn std::error::Error>> {
    let q = "select isbn, publisher from textbooks";
    let query = sqlx::query_as::<_, Textbook>(q).fetch_all(conn).await?;
    Ok(query)
}

pub async fn update_textbook_publisher(
    conn: &sqlx::PgPool,
    isbn: &str,
    publisher: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"update textbooks set publisher=$1
    where isbn=$2"#;
    sqlx::query(q)
        .bind(publisher)
        .bind(isbn)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_textbook(
    conn: &sqlx::PgPool,
    textbook: Textbook,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"insert into textbooks (isbn, publisher) values($1, $2)"#;
    sqlx::query(q)
        .bind(&textbook.isbn)
        .bind(&textbook.publisher)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn remove_textbook(
    conn: &sqlx::PgPool,
    studentid: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"delete from students
    where studentid=$1"#;
    sqlx::query(q).bind(studentid).execute(conn).await?;
    Ok(())
}
