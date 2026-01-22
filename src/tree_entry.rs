use crate::mode::Mode;

pub struct TreeEntry {
    pub mode: Mode,
    pub name: String,
    pub hash: [u8; 20],
}

impl std::fmt::Display for TreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TreeEntry { mode, name, hash } = self;
        write!(f, "{mode} {name} {hash:x?}")
    }
}
