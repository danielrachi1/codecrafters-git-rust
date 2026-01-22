use crate::object::Object;
use crate::utils;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub struct BlobObject(pub String);

impl Object for BlobObject {
    fn r#type(&self) -> String {
        "blob".to_string()
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn store(&self) -> String {
        format!("blob {}\0{}", self.size(), self.0)
    }

    fn hashed_store(&self) -> String {
        utils::hash(&self.store())
    }

    fn compressed_store(&self) -> Vec<u8> {
        utils::compress(&self.store())
    }

    fn read_db(hash: &str) -> Self {
        let (dir, file) = hash.split_at(2);
        let path = Path::new(".git/objects").join(dir).join(file);

        let compressed_store = std::fs::read(path).unwrap();
        let mut decoder = ZlibDecoder::new(&compressed_store[..]);
        let mut decompressed_store = String::new();
        decoder.read_to_string(&mut decompressed_store).unwrap();

        let (_header, content) = decompressed_store.split_once("\0").unwrap();

        BlobObject(content.to_string())
    }

    fn write_db(&self) {
        let hashed_store = &self.hashed_store();
        let (dir, file) = hashed_store.split_at(2);

        let mut path = PathBuf::from(".git").join("objects").join(dir);
        fs::create_dir(&path).unwrap();

        path.push(file);

        fs::write(path, self.compressed_store()).unwrap();
    }
}

impl BlobObject {
    pub fn new(content: &str) -> Self {
        Self(content.to_string())
    }
}
