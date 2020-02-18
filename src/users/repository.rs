#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::users;
use crate::users::Users;

pub fn insert(users: Users, connection: &PgConnection) -> QueryResult<Users> {
    diesel::insert_into(users::table)
        .values(&InsertableUsers::from_users(users))
        .get_result(connection)
}

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUsers {
    pub id: i32,
    pub name: String,
    pub username: String
}

impl InsertableUsers {

    fn from_users(users: Users) -> InsertableUsers {
        InsertableUsers {
            id: users.id,
            name: users.name,
            username: users.username
        }
    }
}