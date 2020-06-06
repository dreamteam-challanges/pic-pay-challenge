use crate::controllers::users;
use crate::diplomat::db::DbExecutor;
use actix::prelude::Addr;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(users::create);
}

#[derive(Clone)]
pub struct Clients {
    pub postgres: Addr<DbExecutor>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            postgres: DbExecutor::address(),
        }
    }
}
