mod api;
mod conversion;
mod database;

#[macro_use]
extern crate rocket;

use api::*;
use conversion::{base64_to_zip, zip_to_base64};

use std::io;
use tokio::fs;
use tokio::runtime::Runtime;
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
        .mount("/", routes![world, test, package_rate])
        .manage(database::module_db().await)
}

/*
fn main() -> io::Result<()> {
    // Create a new Tokio runtime
    let rt = Runtime::new()?;

    // Run the async function using the runtime
    rt.block_on(async {
        let base64_str = "SGVsbG8sIHdvcmxkIQ=="; // "Hello, world!" in base64
        let path =
            "C:/Users/User/Desktop/Purdue/ECE46100/Project2/module-registry/src/output/output.txt";

        base64_to_zip(base64_str, path).await?;
        let output = zip_to_base64(
            "C:/Users/User/Desktop/Purdue/ECE46100/Project2/module-registry/src/output/output.txt",
        )
        .await
        .unwrap();
        println!("{}", output);

        Ok(())
    })
}*/
