pub mod todo;

use crate::todo_mgr::todo::Todo;

use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::cmp::{max};

#[derive(Serialize, Deserialize)]
pub struct TodoMgr{
    todos: Vec<Todo>,
    next_id: i64,
}

impl TodoMgr{
    pub fn new() -> Self{
        Self{
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load() -> Self{
        match fs::read_to_string("data/todos.json"){
            Ok(json) => {
                serde_json::from_str(&json).unwrap()
            },
            Err(_) => {Self::new()},
        }
    }

    pub fn save(&self){
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create("data/todos.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn add_todo(&mut self, todo: Todo){
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn get_next_id(&self) -> i64{
        self.next_id
    }

    pub fn change_todo(&mut self, todo: Todo){
        if let Some(target) = self.todos.iter().position(|t| t.id == todo.id){
            self.todos[target] = todo;
        }
    }

    pub fn toggle_complete_by_id(&mut self, id: i64){
        let now = Local::now().timestamp();
        if let Some(target) = self.todos.iter().position(|t| t.id == id){
            self.todos[target].is_complete ^= true;
            if self.todos[target].is_complete{
                self.todos[target].complete_time = Some(now);
            }else{
                self.todos[target].complete_time = None;
            }
        }
    }

    pub fn delete_todo(&mut self, id: i64){
        self.todos.retain(|t| t.id != id);
    }

    pub fn get_todos(&self) -> &[Todo] {
        &self.todos
    }

    pub fn update(&mut self){
        let now = Local::now().timestamp();
        for todo in self.todos.iter_mut(){
            todo.remain = max(todo.limit_time - now, 0);
        }
    }
}