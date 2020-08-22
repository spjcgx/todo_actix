use actix_web::{HttpServer, App, web, Responder};
mod models;

async fn index() -> impl Responder {
    web::HttpResponse::Ok().json(
        models::Status {
            status: "OK".to_string(),
        })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
