use std::env;
use actix_web::{ HttpResponse };
use mongodb::{bson::doc, Client};
use rand::Rng;
use crate::models::{error::Error, success::Success, word::Word, word_array::WordArray};
use futures_util::stream::TryStreamExt;

pub async fn post_word_doc(data: Word, mongo_client: &Client) -> HttpResponse {
    let db = env::var("DB_NAME").unwrap_or_else(|_| "".into());
    let collection = env::var("COLLECTION_NAME").unwrap_or_else(|_| "".into());
    let collection = mongo_client.database(&db).collection(&collection);
    let word_to_insert = Word {
        name: data.name,
        members: data.members,
        random: Some(rand::thread_rng().gen_range(1..=100))
    };
    let result = collection.insert_one(word_to_insert).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(Success {
            message: "Word Doc Created!".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(Error {
            message: err.to_string(),
        }),
    }
}


pub async fn get_random_word_doc(mongo_client: &Client) -> HttpResponse {
    let db = env::var("DB_NAME").unwrap_or_else(|_| "".into());
    let collection_name = env::var("COLLECTION_NAME").unwrap_or_else(|_| "".into());
    let collection = mongo_client.database(&db).collection::<Word>(&collection_name);
    let random_number = rand::thread_rng().gen_range(1..=100);
    let query = doc! {"random": doc! {"$gt": random_number}};

    let mut cursor = collection.find(query).await.expect("Failed to execute query");
    let mut words: Vec<Word> = Vec::new();

    // Iterate over the cursor and handle each document
    while let Ok(Some(word)) = cursor.try_next().await {
        words.push(word)
    }

    HttpResponse::Ok().json(WordArray{ 
        data: words
    })


}
