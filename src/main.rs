mod api;
mod conversion;
use api::*;
use conversion::*;
#[macro_use]
extern crate rocket;

//#[cfg(test)]
//mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}

/*#[tokio::main]
async fn main() {
    let output = base64_to_zip("SGVsbG8gV29ybGQhPQ==", "./output/output.txt")
        .await
        .unwrap();

    println!("{:?}", output);
}*/
