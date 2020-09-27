#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;

//Database Connection
mod db;

//Schema database for Diesel
mod schema;

//Models
mod models;

use models::tasks::{Task, ETask};

#[post("/",data="<data>")]
fn create_task(data: Json<ETask>, connection: db::Connection) -> Json<bool> {
    Json(ETask::create(&data,&connection))
}

#[get("/")]
fn get_tasks(connection: db::Connection) -> Json<Vec<Task>> {
    Json(Task::select_all(&connection))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .manage(db::connect())
    .mount("/", routes![create_task,get_tasks])
}

fn main(){
    rocket().launch();
}
