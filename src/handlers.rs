use actix_web::{web, Responder};
use crate::models;
use deadpool_postgres::{Pool, Client};
use crate::db;
use crate::errors::{AppError};

pub async  fn index() -> impl Responder {
    web::HttpResponse::Ok().json(
        models::Status {
            status: "OK".to_string(),
        })
}
pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client:Client = db_pool.get()
    .await
    .map_err(AppError::db_error)?;
    let result = db::get_todos(&client).await;
   result.map(|todos| web::HttpResponse::Ok().json(todos))

}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client:Client = db_pool.get().await.expect("Error connect to the database.");
    let resut = db::get_items(&client, path.0).await;
    match resut {
        Ok(list) => web::HttpResponse::Ok().json(list),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<models::CreateTodoList>) -> Result<impl Responder, AppError> {
    let client:Client = db_pool.get()
    .await
    .map_err(AppError::db_error)?;

    let result = db::create_todo(&client, json.title.clone()).await;
    result.map(|todo|web::HttpResponse::Ok().json(todo))
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>)->Result<impl Responder, AppError>{
    let client:Client = db_pool.get()
    .await
    .map_err(AppError::db_error)?;

    let result = db::check_item(&client, path.0, path.1).await;
    result.map(|updated| web::HttpResponse::Ok().json(models::ResultResponse{success: updated}))
}