use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub username: String
}

impl User{
    pub fn create(user: User, connection: &pgConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    } 
}