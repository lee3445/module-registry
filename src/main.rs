mod api;
mod database;

#[macro_use]
extern crate rocket;

use api::*;

//#[cfg(test)]
//mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world, test, package_rate, package_rate_bad]).manage(database::module_db())
}
