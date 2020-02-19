#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::users;

pub mod handler;
//pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub username: String
}