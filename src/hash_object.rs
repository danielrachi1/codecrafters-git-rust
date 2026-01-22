use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use std::{fs, io::Write};

pub fn store(content: &str) -> String {
    // Hardcoding it for now, will need more types later
    let blob = "blob";
    let size = content.len();
    format!("{blob} {size}\0{content}")
}

pub fn hash(data: &str) -> String {
    let hash = Sha1::digest(data.as_bytes());
    format!("{hash:x}")
}

pub fn compress(data: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

pub fn write(hash: &str, compressed_data: Vec<u8>) {
    let (dir, file) = hash.split_at(2);

    let mut path = PathBuf::from(".git").join("objects").join(dir);
    fs::create_dir(&path).unwrap();

    path.push(file);

    fs::write(path, compressed_data).unwrap();
}
