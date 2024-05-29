use std::error::Error;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod db;
mod mydb;
mod routes;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_db().await?;
    Ok(())
}

async fn test_db() -> Result<(), Box<dyn Error>> {
    HttpServer::new(|| {
        App::new()
            .service(routes::get::get_student)
            .service(routes::get::get_students)
            .service(routes::get::get_books)
            .service(routes::get::get_reservations)
            .service(routes::get::get_orders)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
