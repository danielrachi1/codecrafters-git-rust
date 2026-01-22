use flate2::bufread::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub fn file_contents(hash: &str) -> Vec<u8> {
    let (dir, file) = hash.split_at(2);
    let path = PathBuf::from(".git/objects").join(dir).join(file);
    fs::read(path).unwrap()
}

pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(&data[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}
