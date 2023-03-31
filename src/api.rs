//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!\n"
}
#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test3!\n"
}
#[get("/package/<id>")]
pub fn package(id:String) -> &'static str {
    "Hello, package3\n"
}
