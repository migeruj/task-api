use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

#[derive(Deserialize,Serialize,Insertable)]
#[table_name = "tasks"]
pub struct ETask {
    pub name: String,
    pub done: Option<bool>
}
#[derive(AsChangeset, Serialize, Deserialize,Identifiable,Queryable,Debug)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub update_at: NaiveDateTime
}

impl ETask {
    pub fn create(data: &ETask, connection: &MysqlConnection) -> bool {
        diesel::insert_into(tasks::table)
            .values(data)
            .execute(connection).is_ok()
    }
}

impl Task {
    pub fn select_all(connection: &MysqlConnection) -> Vec<Task> {
        tasks::table.load::<Task>(connection).unwrap()
    }
}