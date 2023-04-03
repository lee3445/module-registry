//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
pub fn world() -> &'static str {
    "Hello, world5!\n"
}
#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test5!\n"
}
#[get("/package/<id>")]
pub fn package(id:String) -> &'static str {
    "Hello, package5\n"
}
