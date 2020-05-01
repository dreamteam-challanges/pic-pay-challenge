use crate::controllers::users;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        users::create
    );
}