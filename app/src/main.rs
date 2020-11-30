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
    Json(Task::all_task(&connection))
}

#[get("/<id>")]
fn get_tasks_by_id(id: i32, connection: db::Connection) -> Json<Vec<Task>> {
    Json(Task::select_by_id(&id, &connection))
}

#[patch("/<id>", data="<data>")]
fn update_task(id: i32, data: Json<ETask>,connection: db::Connection) -> Json<bool> {
    Json(Task::update(&id,&data, &connection))
}

#[delete("/<id>")]
fn delete_task(id: i32, connection: db::Connection) -> Json<bool> {
    Json(Task::delete(&id, &connection))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .manage(db::connect())
    .mount("/task", routes![create_task,get_tasks,get_tasks_by_id,update_task,delete_task])
}

fn main(){
    rocket().launch();
}
