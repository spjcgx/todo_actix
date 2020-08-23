use actix_web::{web, Responder};
use crate::models;
use deadpool_postgres::{Pool, Client};
use crate::db;

pub async  fn index() -> impl Responder {
    web::HttpResponse::Ok().json(
        models::Status {
            status: "OK".to_string(),
        })
}
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client:Client = db_pool.get().await.expect("Error connection to the database.");
    let result = db::get_todos(&client).await;
    match result {
        Ok(list) => web::HttpResponse::Ok().json(list),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }

}