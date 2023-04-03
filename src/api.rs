//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
pub fn world() -> &'static str {
    "Hello, world4!\n"
}
#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test4!\n"
}
#[get("/package/<id>")]
pub fn package(id:String) -> &'static str {
    "Hello, package4\n"
}
