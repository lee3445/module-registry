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
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .mount(
            "/",
            routes![
                world,
                test,
                package_create,
                package_retrieve,
                package_reset,
                package_delete,
                authenticate,
                packages_list,
                packages_list_bad_offset,
                packages_list_400,
                package_rate,
                package_update,
                package_by_name_get,
                package_by_name_delete,
                package_by_regex_get,
            ],
        )
        .register("/", catchers![redirect_422_to_400])
        .manage(database::module_db().await)
}
