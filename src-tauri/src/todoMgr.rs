pub mod todo;

use crate::todoMgr::todo::Todo;

pub struct TodoMgr{
    todos: Vec<Todo>,
}

impl TodoMgr{
    pub fn new() -> Self{
        Self{
            todos: Vec::new(),
        }
    }

    pub fn add_todo(&mut self, todo: Todo){

    }
}