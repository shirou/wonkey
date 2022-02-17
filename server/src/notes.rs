use actix_web::{HttpResponse, Responder};

pub async fn list_api() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn create_api(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn list() {

}