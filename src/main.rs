#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
use rocket_contrib::Json;

mod db;
mod schema;

mod users;
use user::User;

mod tweets;
use tweet::Tweet;

#[post("/user/", data = "<user>")]
fn create_user(user: Json<User>, connection: db::Connection) -> Json<User> {
    let insert = User { id:None, ..user.into_inner()};
    Json(User::create(insert, &connection))
}

#[post("/tweets/", data = "<tweet>")]
fn create_tweet(tweet: Json<Tweet>, connection: db::Connection) -> Json<Tweet> {
    let insert = Tweet { id:None, ..tweet.into_inner()};
    Json(Tweet::create(insert, &connection))
}

#[get("/tweets/")]
fn read_tweets(connection: db::Connection) -> Json<Value> {
    Json(json!(Tweet::read(&connection)))
}

#[get("/tweet/<id>")]
fn read_tweets(connection: db::Connection) -> Json<Value> {
    let get = Tweet { id: Some(id) };
    Json(json!(Tweet::get(id,&connection)))
}

fn main() {
    rocket::ignite()
    .manage(db::connect())
    .mount("/user/", routes![create_user])
    .mount("/tweets/", routes![create_tweet,read_tweets])
    .mount("/tweet/", routes![read_tweet])
    .launch();
}
