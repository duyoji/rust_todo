#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
extern crate dotenv;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use schema::todos;

use self::models::{NewTodo, Todo, UpdateTodo};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn fetch_todos() -> Result<Vec<Todo>, String> {
    let connection = establish_connection();
    todos::table
        .load::<Todo>(&connection)
        .map_err( |err| err.to_string() )
}

pub fn create_todo(new_todo: NewTodo) -> Result<Todo, String> {
    let connection = establish_connection();
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&connection)
        .map_err( |err| err.to_string() )
}

pub fn update_todo(id: i32, params: UpdateTodo) -> Result<Todo, String>  {
    let connection = establish_connection();

    diesel::update(todos::table.filter(todos::id.eq(id)))
        .set(
            (
                todos::title.eq(params.title),
                todos::body.eq(params.body),
                todos::completed.eq(params.completed)
            )
        )
        .get_result(&connection)
        .map_err( |err| err.to_string() )
}

pub fn delete_todo(id: i32) -> Result<usize, String>  {
    let connection = establish_connection();

    diesel::delete(todos::table.filter(todos::id.eq(id)))
        .execute(&connection)
        .map_err( |err| err.to_string() )
}