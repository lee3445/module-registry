use std::path::Path;
use rocket::fs::{NamedFile, relative};
//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
pub async fn world() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("index.html"))).await.ok()
}
#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test3!\n"
}
#[get("/package/<id>")]
pub fn package(id:String) -> &'static str {
    "Hello, package3\n"
}
