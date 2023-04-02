use base64::{engine::general_purpose, Engine as _};
use rocket::tokio::fs;
use rocket::tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

pub async fn base64_to_zip(base64_str: &str, path: &str) -> io::Result<()> {
    // remove padding
    let unpadded_str = base64_str.trim_end_matches('=');

    // decode base64
    let decoded_bytes = general_purpose::STANDARD_NO_PAD
        .decode(unpadded_str)
        .unwrap();

    // write to a file
    fs::write(path, decoded_bytes).await?;

    Ok(())
}

async fn file_to_base64(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(general_purpose::STANDARD.encode(&buffer))
}

/*#[cfg(test)]
mod tests {
    use super::*;

    // test base64 conversion
    #[rocket::async_test]
    async fn test1() {
        base64_to_zip("SGVsbG8gV29ybGQhPQ==", "./output/output.txt")
            .await
            .unwrap();
        let mut file = fs::File::open("./output/output.txt").await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();
        assert!(contents == "Hello, world!");
    }
}*/
