mod api;
use api::*;
#[macro_use]
extern crate rocket;

//#[cfg(test)]
//mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world, test])
}
