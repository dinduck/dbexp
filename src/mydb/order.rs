use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    pub orderid: i32,
    pub publisher: String,
    pub isbn: String,
    pub quantity: i32,
    pub orderdate: chrono::NaiveDate,
}

pub async fn read_orders(conn: &sqlx::PgPool) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let q = "select orderid, publisher, isbn, quantity, orderdate from orders";
    let query = sqlx::query_as::<_, Order>(q).fetch_all(conn).await?;
    Ok(query)
}

pub async fn export_orders(
    conn: &sqlx::PgPool,
    month: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let q = format!(
        r#"INSERT INTO Orders (Publisher, ISBN, Quantity, OrderDate)
    SELECT t.Publisher, r.ISBN, SUM(r.Quantity) AS TotalQuantity, CURRENT_DATE
    FROM Reservations r
    JOIN Textbooks t ON r.ISBN = t.ISBN
    WHERE DATE_TRUNC('month', r.REsDate) = '2024-0{}-01'::DATE
    GROUP BY t.Publisher, r.ISBN;"#,
        month
    );
    sqlx::query(&q).execute(conn).await?;
    Ok(())
}
