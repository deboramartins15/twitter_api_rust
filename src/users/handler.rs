use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::users;
use crate::users::Users;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<users>")]
pub fn post(users: Json<Users>, connection: DbConn) -> Result<status::Created<Json<Users>>, Status> {
    users::repository::insert(users.into_inner(), &connection)
        .map(|users| user_created(users))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

fn user_created(users: Users) -> status::Created<Json<Users>> {
    status::Created(
        format!("{host}:{port}/user/{id}", host = host(), port = port(), id = users.id).to_string(),
        Some(Json(users)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}
