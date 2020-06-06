use crate::diplomat::db::{self, DbExecutor};

use actix::prelude::*;
use diesel::{prelude::*, result::QueryResult, PgConnection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    cpf: String,
    telephone: String,
    email: String,
    password: String,
}

impl Message for User {
    type Result = QueryResult<()>;
}

impl Handler<User> for DbExecutor {
    type Result = QueryResult<()>;

    fn handle(&mut self, msg: User, _: &mut Self::Context) -> Self::Result {
        //let user = adapter::auth::signup_to_hash_user(msg);

        db::insert_new_user(user, &self.0.get().expect("Failed to open connection"))
    }
}
