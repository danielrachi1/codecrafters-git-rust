pub trait Object {
    fn r#type(&self) -> String;
    fn size(&self) -> usize;
    fn store(&self) -> String;
    fn hashed_store(&self) -> String;
    fn compressed_store(&self) -> Vec<u8>;
    fn read_db(hash: &str) -> Self;
    fn write_db(&self);
}
