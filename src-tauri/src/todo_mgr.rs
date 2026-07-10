pub mod todo;

use crate::todo_mgr::todo::Todo;

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

    pub fn delete_todo(&mut self, id: i64){
        self.todos.retain(|t| t.id != id);
    }

    pub fn get_todos(&self) -> &[Todo] {
        &self.todos
    }
}