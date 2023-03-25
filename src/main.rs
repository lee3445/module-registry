mod api;
mod database;

#[macro_use]
extern crate rocket;

use api::*;

//#[cfg(test)]
//mod tests;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![world, test, package_rate]).manage(database::module_db().await)
}
