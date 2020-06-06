use actix_web::{web, post, Responder, HttpResponse};
use crate::schemata::{User,UserResponse};
use crate::config::Clients;
use diesel::result::{Error, DatabaseErrorKind};

#[post("/users")]
pub async fn create(data: web::Data<Clients>, info: web::Json<User>) -> impl Responder {
    let code500 = HttpResponse::InternalServerError().finish();
    let resp = data.postgres.send(info.into_inner()).await;

    match resp {
        Ok(query) => match query {
            Ok(id) => HttpResponse::Created().json(UserResponse::new(id)),
            Err(e) => match e {
                Error::DatabaseError(dbe, msg) => match dbe {
                    DatabaseErrorKind::UniqueViolation => HttpResponse::BadRequest().body(String::from(msg.message())),
                    _ => code500,
                },
                _ => code500
            }
        },
        Err(_) => {
            code500
        }, 
    }
}
