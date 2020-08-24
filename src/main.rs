mod models;
mod config;
mod handlers;
mod db;
mod errors;

use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use slog::{Logger, Drain, o, info};
use slog_term;
use slog_async;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();    
    let cfg = config::Config::from_env().unwrap();
    let log = configure_log();
    info!(log,"Starting server at http://{}:{}/", cfg.server.host, cfg.server.port); 
    
    let pool = cfg.pg.create_pool(NoTls).unwrap();


    HttpServer::new(move||{
        App::new()
            .data(
                models::AppState {
                    pool: pool.clone(),
                    log: log.clone(),
                }
            )
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
