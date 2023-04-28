use base64::{engine::general_purpose, Engine as _};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use tokio::fs::File;

pub async fn base64_to_zip(base64_str: &str, path: &str) -> Result<(), ()> {
    // remove padding
    let unpadded_str = base64_str.trim_end_matches('=');

    // decode base64
    let decoded_bytes = general_purpose::STANDARD_NO_PAD
        .decode(unpadded_str)
        .map_err(|_| ())?;

    // write to a file
    fs::write(path, decoded_bytes).await.map_err(|_| ())?;

    Ok(())
}

pub async fn zip_to_base64(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(general_purpose::STANDARD.encode(&buffer))
}
