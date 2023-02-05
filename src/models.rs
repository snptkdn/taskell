use crate::schema::users;
use crate::schema::login_info;
use crate::schema::tasks;
use crate::schema::done_tasks;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub encrypted_pass: String,
}

#[derive(Insertable)]
#[table_name = "login_info"]
pub struct NewLoginInfo {
    pub mac_address: String,
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub point: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "done_tasks"]
pub struct NewDoneTask {
    pub point: Option<i32>,
    pub done_date: NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub encrypted_pass: String,
}

#[derive(Debug, Queryable)]
pub struct LoginInfo {
    pub id: u64,
    pub mac_address: String,
    pub user_id: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct RawTask {
    pub id: u64,
    pub point: Option<i32>,
    pub user_id: Option<i32>,
    pub title: String,
}
