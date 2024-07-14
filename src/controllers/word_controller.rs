use std::env;

use actix_web::{web, HttpResponse};
use mongodb::{bson::{bson, Bson}, Client, Collection};

use crate::models::{success::Success, word::Word, error::Error};


pub async fn post_word_doc(data: Word, mongo_client: &Client) -> () {
    
    let db = env::var("DB_NAME").unwrap_or_else(|_| "".into());
    let collection = env::var("COLLECTION_NAME").unwrap_or_else(|_| "".into());
    
    let collection = mongo_client.database(&db).collection(&collection);
    let result = collection.insert_one(data).await;
    
    let success_msg = Success {
        message: "Word Doc Created!".to_string()
    };
    
    match result {
        Ok(_) => HttpResponse::Ok().json(&success_msg),
        Err(err) => HttpResponse::InternalServerError().json(Error{ message: err.to_string() }),
    };

}
