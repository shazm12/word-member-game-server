use crate::controllers::word_controller::get_random_word_doc;
use crate::models::word::Word;
use crate::{controllers::word_controller::post_word_doc, AppState};
use actix_web::{get, post, web, Responder};

#[post("/create")]
async fn handle_word(item: web::Json<Word>, app_state: web::Data<AppState>) -> impl Responder {
    let mongo_client = &app_state.mongo_client;
    let response = post_word_doc(item.into_inner(), &mongo_client).await;
    response
}

#[get("/get")]
async fn get_random_word(app_state: web::Data<AppState>) -> impl Responder {
    let mongo_client = &app_state.mongo_client;
    let response = get_random_word_doc(&mongo_client).await;
    response
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/word")
                    .service(handle_word)
                    .service(get_random_word));
}
