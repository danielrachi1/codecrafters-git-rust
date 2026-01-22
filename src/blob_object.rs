use crate::object::Object;
use crate::utils;
use std::fs;
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
        let compressed_store = utils::file_contents(hash);
        let decompressed_store = utils::decompress_to_string(compressed_store);

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
