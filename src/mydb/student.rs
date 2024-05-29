use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Student {
    pub studentid: i32,
    pub studentname: String,
    pub email: String,
}

pub async fn read_students(
    conn: &sqlx::PgPool,
) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    let q = "select studentid, studentname, email from students";
    let query = sqlx::query_as::<_, Student>(q).fetch_all(conn).await?;
    Ok(query)
}

pub async fn read_students_by_id(
    conn: &sqlx::PgPool,
    id: i32,
) -> Result<Student, Box<dyn std::error::Error>> {
    let q = r#"select studentid, studentname, email from students where studentid=$1"#;
    let query = sqlx::query_as::<_, Student>(q)
        .bind(id)
        .fetch_one(conn)
        .await?;
    Ok(query)
}

pub async fn update_student_name(
    conn: &sqlx::PgPool,
    id: i32,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"update students set studentname=$1
    where studentid=$2"#;
    sqlx::query(q).bind(name).bind(id).execute(conn).await?;
    Ok(())
}

pub async fn add_student(
    conn: &sqlx::PgPool,
    student: Student,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"insert into students (studentid, studentname, email) values($1, $2, $3)"#;
    sqlx::query(q)
        .bind(&student.studentid)
        .bind(&student.studentname)
        .bind(&student.email)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_student_default(
    conn: &sqlx::PgPool,
    student: Student,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"insert students (studentname, email) values($1, $2)"#;
    sqlx::query(q)
        .bind(&student.studentname)
        .bind(&student.email)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn remove_student(
    conn: &sqlx::PgPool,
    studentid: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"delete from students
    where studentid=$1"#;
    sqlx::query(q).bind(studentid).execute(conn).await?;
    Ok(())
}
