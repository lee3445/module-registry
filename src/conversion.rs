use base64::{engine::general_purpose, Engine as _};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io;

pub fn base64_to_zip(base64_str: &str, path: &str) -> std::io::Result<()> {
    let decoded_bytes = general_purpose::STANDARD_NO_PAD.decode(base64_str).unwrap();
    let mut decoder = GzDecoder::new(&decoded_bytes[..]);
    let mut file = File::create(path)?;

    io::copy(&mut decoder, &mut file)?;
    Ok(())
}
