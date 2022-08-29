//! Our application state and relevant methods
//!
//! Druid uses `Data` to know when it should re-render

use crate::delegate::DELETE;
use druid::{im::Vector, Data, Env, EventCtx, Lens};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Error};
use uuid::Uuid;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    new_todo: String,
    todos: Vector<TodoItem>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self {
            new_todo: "".into(),
            todos: Vector::from(todos),
        }
    }

    fn add_todo(&mut self) {
        self.todos.push_front(TodoItem::new(&self.new_todo));
        self.new_todo = "".into();
        self.save_to_json().unwrap();
    }

    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_todo()
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let todo_vec: Vec<TodoItem> = self.todos.iter().map(|item| item.to_owned()).collect();
        let serialized = serde_json::to_string_pretty(&todo_vec)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn load_from_json() -> Self {
        let file = std::fs::File::open("todos.json");
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or_default();
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                }
            }
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
            },
        }
    }

    pub fn clear_completed(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.todos.retain(|item| !item.done);

        data.save_to_json().unwrap();
    }

    pub fn delete_todo(&mut self, id: &Uuid) {
        self.todos.retain(|item| &item.id != id);

        self.save_to_json().unwrap();
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct TodoItem {
    #[data(same_fn = "PartialEq::eq")]
    pub id: Uuid,
    pub done: bool,
    pub text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            done: false,
            text: text.into(),
        }
    }

    pub fn click_delete(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        ctx.submit_command(DELETE.with(data.id));
    }
}
