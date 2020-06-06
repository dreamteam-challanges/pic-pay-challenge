use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    prelude::*,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::QueryResult,
};
use std::env;
use crate::modelata::User;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn address() -> Addr<DbExecutor> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        SyncArbiter::start(4, move || DbExecutor(pool.clone()))
    }
}

pub fn insert_new_user(user: User, conn: &PgConnection) -> QueryResult<uuid::Uuid> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(&user).execute(conn)?;

    Ok(user.id)
}
