use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Reservation {
    pub reservationid: i32,
    pub studentid: i32,
    pub isbn: String,
    pub resdate: chrono::NaiveDate,
    pub quantity: i32,
}

pub async fn read_reservations(
    conn: &sqlx::PgPool,
) -> Result<Vec<Reservation>, Box<dyn std::error::Error>> {
    let q = "select reservationid, studentid, isbn, resdate, quantity from reservations";
    let query = sqlx::query_as::<_, Reservation>(q).fetch_all(conn).await?;
    Ok(query)
}

pub async fn add_reservation(
    conn: &sqlx::PgPool,
    reservation: Reservation,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"insert into reservations (reservationid, studentid, isbn, resdate, quantity) values($1, $2, $3, $4, $5)"#;
    sqlx::query(q)
        .bind(reservation.reservationid)
        .bind(reservation.studentid)
        .bind(&reservation.isbn)
        .bind(&reservation.resdate)
        .bind(reservation.quantity)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn add_reservation_default(
    conn: &sqlx::PgPool,
    reservation: Reservation,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = r#"insert into reservations (studentid, isbn, quantity) values($1, $2, $3)"#;
    sqlx::query(q)
        .bind(reservation.studentid)
        .bind(&reservation.isbn)
        .bind(reservation.quantity)
        .execute(conn)
        .await?;
    Ok(())
}
