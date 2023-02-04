use crate::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub encrypted_pass: String,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub encrypted_pass: String,
}
