use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::io::Write;

pub fn hash(data: &str) -> String {
    let hash = Sha1::digest(data.as_bytes());
    format!("{hash:x}")
}

pub fn compress(data: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}
