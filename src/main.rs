#![allow(unused)]
#[macro_use] extern crate rocket;

mod routes;
mod db;

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::index,

            routes::get_all_tasks,
            routes::get_task,

            routes::new_task,
            routes::delete_task,
            routes::edit_task,

            routes::finish_task,
            routes::definish_task,
        ])
}
