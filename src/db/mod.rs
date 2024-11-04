use std::sync::LazyLock;
use surrealdb::{
    Surreal,
    opt::auth::Root,
    engine::remote::ws::{
        Ws,
        Client
    }
};

mod models;

type DBClient = Surreal<Client>;
const TABLE_NAME: &str = "tasks";

pub async fn db_init() -> LazyLock<DBClient> {
    LazyLock::new(Surreal::init)
}

pub async fn establish_connection(db_client: DBClient) {
    let _ = dotenvy::dotenv().ok();

    let url = std::env::var("DB_URL").expect("Unable to get DB_URL variable!");
    let username = std::env::var("DB_USERNAME").expect("Unable to get DB_USERNAME variable!");
    let password = std::env::var("DB_PASSWORD").expect("Unable to get DB_PASSWORD variable!");
    let namespace = std::env::var("DB_NS").expect("Unable to get DB_NS variable!");
    let database = std::env::var("DB_DATABASE").expect("Unable to get DB_DATABASE variable!");

    db_client.connect::<Ws>(url).await.expect("Unable to connect to Database!");
    db_client.signin(Root {
        username: username.as_str(),
        password: password.as_str()
    }).await.expect("Unable to signin! Please checkout username and password!");

    db_client.use_ns(namespace).use_db(database).await.expect("An error occured with using namespace or database!");
}

// task managament

pub async fn create_task(db_client: DBClient, title: &'static str, description: &'static str) -> surrealdb::Result<Option<models::Record>> {
    let created: Option<models::Record> = db_client
        .create(TABLE_NAME)
        .content(
            models::Task {
                title,
                description,
                finished: false
            }
        ).await?;

    Ok(created)
}
