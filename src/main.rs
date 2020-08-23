mod models;
mod config;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();    
    let cfg = config::Config::from_env().unwrap();
    println!("Starting server at http://{}:{}/", cfg.server.host, cfg.server.port); 
    
    let pool = cfg.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move||{
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(handlers::index))
            .route("/todos{_:/?}", web::get().to(handlers::get_todos))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(handlers::get_items))
            .route("/todos{_:/?}", web::post().to(handlers::create_todo))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(handlers::check_item))
    })
        .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
        .run()
        .await
}
