pub enum FileType {
    FILE,
    DIR,
    SYMLINK,
    OTHER,
}

impl From<std::fs::FileType> for FileType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_dir() {
            Self::DIR
        }
        else if value.is_file() {
            Self::FILE
        }
        else if value.is_symlink() {
            Self::SYMLINK
        }
        else {
            Self::OTHER
        }
    }
}
