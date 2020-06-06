#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod adapter;
mod config;
mod controllers;
mod schemata;
mod schema;
mod modelata;
mod diplomat;

use config::{Clients, routes};

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error>  {
    dotenv().ok();
    
    HttpServer::new(|| {
        App::new()
            .data(Clients::new())
            .configure(routes)
    })
    .workers(6)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
