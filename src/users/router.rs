use crate::users;
use rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/user",
               routes![users::handler::post],
        ).launch();
}