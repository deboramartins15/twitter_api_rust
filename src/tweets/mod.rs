#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::tweets;

pub mod handler;
//pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tweets"]
pub struct Tweets {
    pub id: i32,
    pub user_id: i32,
    pub content: String
}