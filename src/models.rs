use crate::schema::users;
use crate::schema::login_info;

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

#[derive(Debug, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub encrypted_pass: String,
}
