#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate cc2_rust_todo;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_json;

use rocket_contrib::{Json, Value};
use cc2_rust_todo::*;
use cc2_rust_todo::models::{NewTodo, UpdateTodo};

fn format_response<T>(result: Result<T, String>) -> Json<Value>
    where T: serde::Serialize {
    if result.is_ok() {
        Json( json!( { "data": result.unwrap() } ) )
    } else {
        Json( json!( { "data": result } ) )
    }
}

#[get("/")]
fn index() -> Json<Value> {
    let todos_result = fetch_todos();
    format_response(todos_result)
}

#[post("/", format = "application/json", data = "<new_todo>")]
fn post_todo(new_todo: Json<NewTodo>) -> Json<Value> {
    let created_todo_result = cc2_rust_todo::create_todo(
        new_todo.into_inner()
    );
    format_response(created_todo_result)
}

#[put("/<id>", format = "application/json", data = "<update_todo>")]
fn update_todo(id: i32, update_todo: Json<UpdateTodo>) -> Json<Value> {
    let updated_todo = cc2_rust_todo::update_todo(
        id,
        update_todo.into_inner()
    );
    format_response(updated_todo)
}

#[delete("/<id>")]
fn delete_todo(id: i32) -> Json<Value> {
    let deleted_count = cc2_rust_todo::delete_todo(id);
    if deleted_count.is_ok() {
        let is_deleted = match deleted_count.unwrap() {
            1 => true,
            _ => false
        };

        Json( json!( { "data": is_deleted } ) )
    } else {
        Json( json!( { "data": deleted_count } ) )
    }
}

fn main() {
    rocket::ignite().mount(
        "/todos",
        routes![
            index,
            post_todo,
            update_todo,
            delete_todo
        ]
    ).launch();
}