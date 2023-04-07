mod api;
mod conversion;
mod database;

#[macro_use]
extern crate rocket;

use api::*;

//#[cfg(test)]
//mod tests;

#[launch]
async fn rocket() -> _ {
    let port: u32 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(8080);
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "127.0.0.1"));

    rocket::custom(figment)
        .mount("/", routes![world, package_retrieve, package_rate])
        .manage(database::module_db().await)
}
