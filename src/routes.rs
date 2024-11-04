use rocket::serde::json::Json;
use super::db::{self, models};

#[get("/")]
pub async fn index() -> &'static str {
    "TODO API written in Rust using Rocket.rs"
}

#[get("/tasks")]
pub async fn get_all_tasks() -> Json<Vec<models::Task>> {
    let connection = db::establish_connection().await;

    Json(
        db::get_all_tasks(connection).await
    )
}

#[get("/tasks/<id>")]
pub async fn get_task(id: &str) -> Json<Option<models::Task>> {
    let connection = db::establish_connection().await;

    Json(
        db::get_task(connection, id).await
    )
}

#[post("/tasks/new", data = "<new>")]
pub async fn new_task(new: Json<models::NewTask>) -> Result<(), ()> {
    let connection = db::establish_connection().await;

    match db::create_task(connection, new.title.clone(), new.description.clone()).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

#[delete("/tasks/<id>")]
pub async fn delete_task<'a>(id: &'a str) -> Result<(), ()> {
    let connection = db::establish_connection().await;

    match db::delete_task(connection, id).await {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

#[post("/tasks/<id>/edit", data = "<edit>")]
pub async fn edit_task<'a>(id: &'a str, edit: Json<models::EditTask>) -> Result<(), ()> {
    let connection = db::establish_connection().await;

    db::edit_task(
        connection,
        id,
        edit.title.clone(),
        edit.description.clone()
    ).await
}

#[post("/tasks/<id>/finish")]
pub async fn finish_task<'a>(id: &'a str) -> Result<(), ()> {
    let connection = db::establish_connection().await;
    
    db::finish_task(
        connection,
        id
    ).await
}

#[post("/tasks/<id>/definish")]
pub async fn definish_task<'a>(id: &'a str) -> Result<(), ()> {
    let connection = db::establish_connection().await;
    
    db::definish_task(
        connection,
        id
    ).await
}
