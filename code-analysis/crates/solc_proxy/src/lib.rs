use crate::local_cache::LocalCache;

mod local_cache;

pub fn fetch_solc_binaries() {
    let cache = LocalCache::new();
    println!("Using local cache: {}", cache.root_dir.display());
}
