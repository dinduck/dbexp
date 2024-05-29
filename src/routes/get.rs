use serde::{Deserialize, Serialize};

use crate::{db, mydb};

#[derive(Debug, Serialize, Deserialize)]
struct DataStudent {
    data: Option<Vec<mydb::student::Student>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Id {
    id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
struct DataBook {
    data: Option<Vec<mydb::textbook::Textbook>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reservation {
    reservationid: i32,
    studentid: i32,
    isbn: String,
    resdate: String,
    quantity: i32,
}
#[derive(Debug, Serialize, Deserialize)]
struct DataReservation {
    data: Option<Vec<Reservation>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub orderid: i32,
    pub publisher: String,
    pub isbn: String,
    pub quantity: i32,
    pub orderdate: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct DataOrder {
    data: Option<Vec<Order>>,
}

#[actix_web::get("/get/students")]
pub async fn get_students() -> impl actix_web::Responder {
    let pool = db::connect().await.unwrap();
    let students = mydb::student::read_students(&pool).await.ok();
    actix_web::web::Json(DataStudent { data: students })
}

#[actix_web::get("/get/student")]
pub async fn get_student(info: actix_web::web::Query<Id>) -> impl actix_web::Responder {
    let pool = db::connect().await.unwrap();
    let student = mydb::student::read_students_by_id(&pool, info.id)
        .await
        .ok();
    actix_web::web::Json(student)
}

#[actix_web::get("/get/books")]
pub async fn get_books() -> impl actix_web::Responder {
    let pool = db::connect().await.unwrap();
    let books = mydb::textbook::read_textbooks(&pool).await.ok();
    actix_web::web::Json(DataBook { data: books })
}

#[actix_web::get("/get/reservations")]
pub async fn get_reservations() -> impl actix_web::Responder {
    let pool = db::connect().await.unwrap();
    let res = mydb::reservation::read_reservations(&pool).await.unwrap();
    let res = translate_res(res).await;
    actix_web::web::Json(DataReservation { data: res })
}

#[actix_web::get("/get/orders")]
pub async fn get_orders() -> impl actix_web::Responder {
    let pool = db::connect().await.unwrap();
    let res = mydb::order::read_orders(&pool).await.unwrap();
    let res = translate_orders(res).await;
    actix_web::web::Json(DataOrder { data: res })
}

async fn translate_res(data: Vec<mydb::reservation::Reservation>) -> Option<Vec<Reservation>> {
    let mut ans = vec![];
    for res in data.into_iter() {
        ans.push(Reservation {
            reservationid: res.reservationid,
            studentid: res.studentid,
            isbn: res.isbn,
            resdate: format!("{}", res.resdate),
            quantity: res.quantity,
        })
    }
    return Some(ans);
}

async fn translate_orders(data: Vec<mydb::order::Order>) -> Option<Vec<Order>> {
    let mut ans = vec![];
    for res in data.into_iter() {
        ans.push(Order {
            orderid: res.orderid,
            publisher: res.publisher,
            isbn: res.isbn,
            quantity: res.quantity,
            orderdate: format!("{}", res.orderdate),
        })
    }
    return Some(ans);
}
