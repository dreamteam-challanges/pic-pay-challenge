use actix_web::{web, post, Responder, HttpResponse};
use crate::schemata::User;
use crate::diplomat::DbExecutor;


#[post("/users")]
// pub async fn create(data: web::Data<DbExecutor>, info: web::Json<User>) -> impl Responder {
pub async fn create(info: web::Json<User>) -> impl Responder {
    // let postgres = &data.postgres;
    // postgres.0.get().expect("Failed to open connection"));
    HttpResponse::Created().body("HAHAHAHAHHA")
}