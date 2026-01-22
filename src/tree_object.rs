use crate::mode::Mode;
use crate::object::Object;
use crate::tree_entry::TreeEntry;
use crate::utils;

pub struct TreeObject(pub Vec<TreeEntry>);

impl Object for TreeObject {
    fn r#type(&self) -> String {
        "tree".to_string()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn store(&self) -> String {
        todo!()
    }

    fn hashed_store(&self) -> String {
        todo!()
    }

    fn compressed_store(&self) -> Vec<u8> {
        todo!()
    }

    fn read_db(hash: &str) -> Self {
        let file_content = utils::file_contents(hash);
        let decompressed_data = utils::decompress(file_content);

        let first_null_idx = decompressed_data.iter().position(|&b| b == b'\0').unwrap();
        let _header = std::str::from_utf8(&decompressed_data[..first_null_idx]).unwrap();

        let mut entries: Vec<TreeEntry> = Vec::new();
        let mut rest = &decompressed_data[first_null_idx + 1..];

        while !rest.is_empty() {
            let whitespace_idx = rest.iter().position(|&b| b == b' ').unwrap();
            let mode = Mode::from_bytes(&rest[..whitespace_idx]).unwrap();

            let null_idx = rest.iter().position(|&b| b == b'\0').unwrap();
            let name = String::from_utf8(rest[whitespace_idx + 1..null_idx].to_vec()).unwrap();

            let hash: [u8; 20] = rest[null_idx + 1..null_idx + 21].try_into().unwrap();

            entries.push(TreeEntry { mode, name, hash });

            rest = &rest[null_idx + 21..];
        }

        Self(entries)
    }

    fn write_db(&self) {
        todo!()
    }
}

impl TreeObject {
    pub fn get_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        for entry in &self.0 {
            names.push(entry.name.as_str());
        }
        names
    }
}
