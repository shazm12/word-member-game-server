use std::env;

use actix_web::HttpResponse;
use mongodb::Client;

use crate::models::{error::Error, success::Success, word::Word};

pub async fn post_word_doc(data: Word, mongo_client: &Client) -> HttpResponse {
    let db = env::var("DB_NAME").unwrap_or_else(|_| "".into());
    let collection = env::var("COLLECTION_NAME").unwrap_or_else(|_| "".into());

    let collection = mongo_client.database(&db).collection(&collection);
    let result = collection.insert_one(data).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(Success {
            message: "Word Doc Created!".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(Error {
            message: err.to_string(),
        }),
    }
}
