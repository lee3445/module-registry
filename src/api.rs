use std::path::Path;
use rocket::fs::{NamedFile, relative};
//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
<<<<<<< HEAD
pub fn world() -> &'static str {
    "Hello, world4!\n"
=======
pub async fn world() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("index.html"))).await.ok()
>>>>>>> 4b26b810433507138c63cba201d4cafbe5ac1a28
}
#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test4!\n"
}
#[get("/package/<id>")]
pub fn package(id:String) -> &'static str {
    "Hello, package4\n"
}
