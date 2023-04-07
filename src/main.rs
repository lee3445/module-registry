mod api;
mod database;
mod conversion;

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
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .mount(
            "/",
            routes![
                world,
                test,
                packages_list,
                packages_list_bad_offset,
                packages_list_400,
                package_rate,
                package_by_name_get,
                package_by_name_delete,
            ],
        )
        .register("/packages", catchers![packages_list_422])
        .manage(database::module_db().await)
}
