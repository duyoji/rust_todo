#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
extern crate cc2_rust_todo;

#[cfg(test)]
mod tests {
  use super::*;
  use cc2_rust_todo::models::Todo;

  describe! stainless {
    describe! fetch_todos {
      it "should return todo list from DB." {
        let result: Result<Vec<Todo>, String> = cc2_rust_todo::fetch_todos();
        let todos: Vec<Todo> = result.unwrap();

        // If you want to see log of println!, run test command with `--nocapture`
        // cargo test -- --nocapture
        println!("@@@@@@@@@ {:?}", todos);

        // This data comes from PostgreSQL DB on Tsuyoshi's laptop.
        // Just play around for now to understand test in Rust.
        assert_eq!(4, todos[0].id);
        assert_eq!("abc2".to_string(), todos[0].title);
        assert_eq!("def2".to_string(), todos[0].body);
        assert_eq!(false, todos[0].completed);
      }
    }
  }
}
