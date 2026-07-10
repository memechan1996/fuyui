//use core::option::Option::Some;
use serde::{Serialize, Deserialize};
//use chrono::{Local, DateTime};

#[derive(Serialize, Deserialize)]
pub struct Todo{
    pub id: i64,
    pub title: String,
    pub is_complete: bool,
    pub remain: i64,
    pub limit_day: i64,
    pub limit_mounth: i64,
    pub limit_hour: i64,
    pub begin_time: i64,
    pub complete_time: Option<i64>,
    pub limit_time: i64,
    pub description: String,
}

