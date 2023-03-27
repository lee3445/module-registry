mod api;
mod conversion;
use api::*;
use conversion::*;
#[macro_use]
extern crate rocket;

//#[cfg(test)]
//mod tests;

/*#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}*/

#[tokio::main]
async fn main() {
    let output = conversion::base64_to_zip(
        "SGVsbG8sIHdvcmxkIQ",
        "C:/Users/User/Desktop/Purdue/ECE46100/Project2/module-registry/src",
    );

    println!("{:?}", output);
}
