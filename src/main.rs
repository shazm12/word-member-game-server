use std::env;

use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use mongodb::Client;
mod controllers;
mod models;
mod routes;

#[derive(Clone)]
struct AppState {
    mongo_client: Client,
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let uri: String = env::var("MONGO_URI").unwrap_or_else(|_| "".into());

    if uri.is_empty() {
        error!("Mongo URI cannot be empty");
        std::process::exit(1);
    }

    let mongo_client: Client = Client::with_uri_str(uri)
        .await
        .expect("failed to connect to DB");

    info!("Connected to DB!");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                mongo_client: mongo_client.clone(),
            }))
            .configure(routes::word_route::init)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?;

    info!("Server started running on http://localhost:8080/");

    server.run().await
}
