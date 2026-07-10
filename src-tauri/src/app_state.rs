
use crate::todo_mgr::TodoMgr;

use std::sync::{Mutex};

pub struct AppState{
    pub todo_mgr: Mutex<TodoMgr>,
}

