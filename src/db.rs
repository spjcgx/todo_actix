use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;


pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, std::io::Error>{
    let statement = client.prepare("SELECT * FROM todo_list ORDER BY id DESC").await.unwrap();
    let todos = client.query(&statement, &[])
        .await  
        .expect("Error get todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, std::io::Error> {
    let stmt = client.prepare("SELECT * FROM todo_item WHERE list_id=$1 ORDER BY id DESC").await.unwrap();
    let items = client.query(&stmt, &[&list_id])
    .await
    .expect("Error get todo items")
    .iter()
    .map(|row| TodoItem::from_row_ref(row).unwrap())
    .collect::<Vec<TodoItem>>();
    Ok(items)
}

pub async fn create_todo(client:&Client, title:String) -> Result<TodoList, std::io::Error> {
    let stmt = client.prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id,title").await.unwrap();
    client.query(&stmt, &[&title])
    .await
    .expect("Error create todo list")
    .iter()
    .map(|row| TodoList::from_row_ref(row).unwrap())
    .collect::<Vec<TodoList>>()
    .pop()
    .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Error creating todo list"))
}