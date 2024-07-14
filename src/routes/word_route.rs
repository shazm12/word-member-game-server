use crate::models::word::Word;
use crate::{controllers::word_controller::post_word_doc, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/create")]
async fn handle_word(item: web::Json<Word>, app_state: web::Data<AppState>) -> impl Responder {
    let mongo_client = &app_state.mongo_client;
    let response = post_word_doc(item.into_inner(), &mongo_client).await;
    HttpResponse::Ok().json(response)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/word").service(handle_word));
}
