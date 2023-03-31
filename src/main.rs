mod api;
mod database;

#[macro_use]
extern crate rocket;

use api::*;

//#[cfg(test)]
//mod tests;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![world, test, packages_list, packages_list_400, package_rate])
        .register("/packages", catchers![packages_list_422])
        .manage(database::module_db().await)
}
