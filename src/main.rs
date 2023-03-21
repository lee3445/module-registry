#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}
