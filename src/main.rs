use actix_web::{App, HttpServer};

mod config;
mod controllers;
mod schemata;
mod diplomat;

use config::{Clients, routes};

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error>  {
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
