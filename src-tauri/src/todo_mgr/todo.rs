//use core::option::Option::Some;

pub struct Todo{
    pub id: i64,
    pub title: String,
    pub is_complete: bool,
    pub remain: i64,
    pub limit: i64,
    pub begin_time: i64,
    pub complete_time: Option<i64>,
    pub limit_time: i64,
    pub description: String,
}

