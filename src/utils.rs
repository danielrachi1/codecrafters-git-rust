use flate2::bufread::ZlibDecoder;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub fn hash(data: &str) -> String {
    let hash = Sha1::digest(data.as_bytes());
    format!("{hash:x}")
}

pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

pub fn decompress_to_string(data: Vec<u8>) -> String {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    decompressed
}

pub fn compress(data: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

pub fn file_contents(hash: &str) -> Vec<u8> {
    let (dir, file) = hash.split_at(2);
    let path = PathBuf::from(".git/objects").join(dir).join(file);
    fs::read(path).unwrap()
}
