use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;

pub mod config;
pub mod handlers;
pub mod models;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let config = config::AppConfig::from_env().expect("Failed to load configuration");

    info!("Server address : {}", config.server.address);
    info!("Server port : {}", config.server.port);

    HttpServer::new(|| App::new().configure(routes::init))
        .bind((config.server.address, config.server.port))?
        .run()
        .await
}
