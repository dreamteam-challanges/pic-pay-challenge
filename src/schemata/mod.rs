use crate::diplomat::db::{self, DbExecutor};
use crate::adapter;

use actix::prelude::*;
use diesel::{result::QueryResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub cpf: String,
    pub telephone: String,
    pub email: String,
    pub password: String,
}

impl Message for User {
    type Result = QueryResult<uuid::Uuid>;
}

impl Handler<User> for DbExecutor {
    type Result = QueryResult<uuid::Uuid>;

    fn handle(&mut self, msg: User, _: &mut Self::Context) -> Self::Result {
        let user = adapter::user(msg);

        db::insert_new_user(user, &self.0.get().expect("Failed to open connection"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    id: uuid::Uuid
}

impl UserResponse {
    pub fn new(id: uuid::Uuid) -> Self {
        Self { id }
    }
}