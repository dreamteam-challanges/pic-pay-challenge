use crate::controllers::users;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .service(
                web::scope("users")
                    .route("", web::post().to(users::create))
            )
    );
}