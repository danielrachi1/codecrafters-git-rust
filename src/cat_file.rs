use flate2::read::ZlibDecoder;
use std::io::Read;
use std::path::Path;

pub fn run(hash: &str) -> (String, usize, String) {
    let (dir, file) = hash.split_at(2);
    let path = Path::new(".git/objects").join(dir).join(file);

    let compressed_store = std::fs::read(path).unwrap();
    let mut decoder = ZlibDecoder::new(&compressed_store[..]);
    let mut decompressed_store = String::new();
    decoder.read_to_string(&mut decompressed_store).unwrap();

    let (header, content) = decompressed_store.split_once("\0").unwrap();
    let (object_type, size) = header.split_once(" ").unwrap();

    (
        object_type.to_string(),
        size.parse().unwrap(),
        content.to_string(),
    )
}
