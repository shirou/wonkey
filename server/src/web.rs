use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::notes;

fn api_v1_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/notes/list")
            .route(web::get().to(notes::list_api))
    )
    .service(
        web::resource("/notes/create")
            .route(web::post().to(notes::create_api))
    )
    ;
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(web::scope("/api/v1/").configure(api_v1_config))
            })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}