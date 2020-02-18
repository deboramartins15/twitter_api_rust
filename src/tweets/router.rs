use crate::tweets;
use rocket;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/tweets",
               routes![tweets::handler::all,
               tweets::handler::get,
               tweets::handler::post],
        ).launch();
}