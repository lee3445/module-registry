mod api;
use api::*;
#[macro_use]
extern crate rocket;

//#[cfg(test)]
//mod tests;

#[launch]
fn rocket() -> _ {
    let port: u32 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(8080);
    let figment = rocket::Config::figment().merge(("port", port));

    rocket::custom(figment).mount("/", routes![world, test, package])
}
