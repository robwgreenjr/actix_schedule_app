#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod api_error;
mod db;
mod schema;
mod staff;
mod store;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(staff::init_routes)
            .configure(store::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}