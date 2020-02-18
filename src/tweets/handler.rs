use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::tweets;
use crate::tweets::Tweets;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Tweets>>, Status> {
    tweets::repository::all(&connection)
        .map(|tweets| Json(tweets))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Tweets>, Status> {
    tweets::repository::get(id, &connection)
        .map(|tweets| Json(tweets))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<tweets>")]
pub fn post(tweets: Json<Tweets>, connection: DbConn) -> Result<status::Created<Json<Tweets>>, Status> {
    tweets::repository::insert(tweets.into_inner(), &connection)
        .map(|tweets| tweets_created(tweets))
        .map_err(|error| error_status(error))
}

fn tweets_created(tweets: Tweets) -> status::Created<Json<Tweets>> {
    status::Created(
        format!("{host}:{port}/tweets/{id}", host = host(), port = port(), id = tweets.id).to_string(),
        Some(Json(tweets)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}