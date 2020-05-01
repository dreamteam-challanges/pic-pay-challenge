use actix_web::{web, Responder, HttpResponse};
use crate::schemata::User;

pub async fn create(info: web::Json<User>) -> impl Responder {
    HttpResponse::Created().body("HAHAHAHAHHA")
}