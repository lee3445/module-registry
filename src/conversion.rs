use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::prelude::*;
use std::{fs, io};

pub async fn base64_to_zip(base64_str: &str, path: &str) -> io::Result<()> {
    // remove padding
    let unpadded_str = base64_str.trim_end_matches('=');

    // decode base64
    let decoded_bytes = general_purpose::STANDARD_NO_PAD
        .decode(unpadded_str)
        .unwrap();

    // write to a file
    fs::write(path, decoded_bytes)?;

    Ok(())
}

pub async fn zip_to_base64(path: &str) -> String {
    let data = fs::read(path).unwrap();
    let base64_str = general_purpose::STANDARD.encode(data);
    base64_str
}

//#[cfg(test)]
/*mod tests {
    use super::*;

    // test base64 conversion
    #[rocket::async_test]
    async fn test1() {
        base64_to_zip("SGVsbG8gV29ybGQhPQ==", "./output/output.txt")
            .await
            .unwrap();
        let mut file = File::open("./output/output.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert!(contents == "Hello, world!");
    }
}*/
