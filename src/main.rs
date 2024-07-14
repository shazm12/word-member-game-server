use std::{env, os::windows::process};

use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, route},
    App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use mongodb::Client;
mod controllers;
mod models;
mod routes;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let uri: String = env::var("MONGO_URI").unwrap_or_else(|_| "".into());

    if uri.is_empty() {
        error!("Mogno URI cannot be empty");
        std::process::exit(1);
    }

    let mongo_client: Client = Client::with_uri_str(uri)
        .await
        .expect("failed to connect to DB");

    info!("Connected to DB!");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(routes::word_route::init)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?;

    info!("Server started running on http://localhost:8080/");

    server.run().await
}