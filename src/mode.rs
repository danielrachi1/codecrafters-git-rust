use std::fmt;

pub enum Mode {
    Regular,
    Executable,
    Symlink,
    Directory,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Mode::Regular => "100644",
            Mode::Executable => "100755",
            Mode::Directory => "040000",
            Mode::Symlink => "120000",
        };
        write!(f, "{s}")
    }
}

impl Mode {
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        match b {
            b"100644" => Some(Self::Regular),
            b"100755" => Some(Self::Executable),
            b"40000" => Some(Self::Directory),
            b"120000" => Some(Self::Symlink),
            _ => None,
        }
    }
}
