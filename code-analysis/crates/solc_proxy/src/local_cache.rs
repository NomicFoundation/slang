use std::path::PathBuf;

pub struct LocalCache {
    pub root_dir: PathBuf,
}

impl LocalCache {
    pub fn new() -> LocalCache {
        LocalCache {
            root_dir: std::env::temp_dir().join("slang-solc-binaries"),
        }
    }
}
