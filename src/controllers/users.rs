use actix_web::{web, post, Responder, HttpResponse};
use crate::schemata::User;


#[post("/users")]
pub async fn create(info: web::Json<User>) -> impl Responder {
    HttpResponse::Created().body("HAHAHAHAHHA")
}