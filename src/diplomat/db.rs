use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    result::QueryResult,
};
use std::env;

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

pub fn insert_new_user(user: User, conn: &PgConnection) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    let new_user = diesel::insert_into(auth_user).values(&user).execute(conn);

    new_user
}
