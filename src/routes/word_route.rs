use crate::controllers::word_controller::get_word_members;
use crate::models::word::Word;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/word")]
async fn handle_word(item: web::Json<Word>) -> impl Responder {
    let response = get_word_members(item.into_inner()).await;
    HttpResponse::Ok().json(response)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_word);
}
