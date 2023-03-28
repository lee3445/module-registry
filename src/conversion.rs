use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::{self, Write};
use zip::{write::FileOptions, ZipWriter};

pub fn base64_to_zip(base64_str: &str, path: &str) -> io::Result<()> {
    let decoded_bytes = general_purpose::STANDARD_NO_PAD.decode(base64_str).unwrap();
    let file = File::create(path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);
    zip.start_file("output.txt", options)?;
    zip.write_all(&decoded_bytes)?;
    zip.finish()?;

    Ok(())
}
