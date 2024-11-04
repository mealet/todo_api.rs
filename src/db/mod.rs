use std::sync::LazyLock;
use surrealdb::{
    Surreal,
    opt::{
        Resource,
        auth::Root
    },
    engine::remote::ws::{
        Ws,
        Client
    }
};

pub mod models;

type DBClient = Surreal<Client>;
const TABLE_NAME: &'static str = "tasks";

pub async fn establish_connection() -> DBClient {
    let _ = dotenvy::dotenv().ok();

    let url = std::env::var("DB_URL").expect("Unable to get DB_URL variable!");
    let username = std::env::var("DB_USERNAME").expect("Unable to get DB_USERNAME variable!");
    let password = std::env::var("DB_PASSWORD").expect("Unable to get DB_PASSWORD variable!");
    let namespace = std::env::var("DB_NS").expect("Unable to get DB_NS variable!");
    let database = std::env::var("DB_DATABASE").expect("Unable to get DB_DATABASE variable!");


    let db_client = Surreal::new::<Ws>(url).await.expect("Unable to connect to Database!");
    db_client.signin(Root {
        username: username.as_str(),
        password: password.as_str()
    }).await.expect("Unable to signin! Please checkout username and password!");

    db_client.use_ns(namespace).use_db(database).await.expect("An error occured with using namespace or database!");

    db_client } // task managament

pub async fn create_task(db_client: DBClient, title: String, description: String) -> surrealdb::Result<Option<models::Record>> {
    let created: Option<models::Record> = db_client
        .create(TABLE_NAME)
        .content(
            models::Task {
                id: None,
                title,
                description,
                finished: false
            }
        ).await?;

    Ok(created)
}

pub async fn delete_task<'a>(db_client: DBClient, id: &'a str) -> surrealdb::Result<Option<models::Record>> {
    let deleted: Option<models::Record> = db_client
        .delete((TABLE_NAME, id))
        .await?;

    Ok(deleted)
}

pub async fn edit_task<'a>(db_client: DBClient, id: &'a str, title: String, description: String) -> Result<(), ()> {
    let edit_task = models::EditTask {
        title,
        description
    };

    match db_client.update::<Option<models::Task>>((TABLE_NAME, id))
        .merge(edit_task)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn finish_task<'a>(db_client: DBClient, id: &'a str) -> Result<(), ()> {
    match db_client.update::<Option<models::Task>>((TABLE_NAME, id))
        .merge(models::CheckFinished {
            finished: true
        }).await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn definish_task<'a>(db_client: DBClient, id: &'a str) -> Result<(), ()> {
    match db_client.update::<Option<models::Task>>((TABLE_NAME, id))
        .merge(models::CheckFinished {
            finished: false 
        }).await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub async fn get_task<'a>(db_client: DBClient, id: &'a str) -> Option<models::Task> {
    match db_client.select::<Option<models::Task>>((TABLE_NAME, id)).await {
        Ok(opt) => opt,
        Err(_) => None
    }
}

pub async fn get_all_tasks(db_client: DBClient) -> Vec<models::Task> {
    let result = db_client.select::<Vec<models::Task>>(TABLE_NAME).await;
    
    match result {
        Ok(list) => list,
        Err(_) => Vec::new()
    }
}
