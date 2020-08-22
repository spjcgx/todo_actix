mod models;
mod config;

use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;

async fn index() -> impl Responder {
    web::HttpResponse::Ok().json(
        models::Status {
            status: "OK".to_string(),
        })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();    
    let cfg = config::Config::from_env().unwrap();
    println!("Starting server at http://{}:{}/", cfg.server.host, cfg.server.port); 
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
        .run()
        .await
}
