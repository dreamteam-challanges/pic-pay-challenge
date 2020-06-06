use actix_web::{web, post, Responder, HttpResponse};
use crate::schemata::User;
use crate::config::Clients;


#[post("/users")]
pub async fn create(data: web::Data<Clients>, info: web::Json<User>) -> impl Responder {
    let resp = data.postgres.send(info.into_inner()).await;
    
    HttpResponse::Created().body("HAHAHAHAHHA")
}
