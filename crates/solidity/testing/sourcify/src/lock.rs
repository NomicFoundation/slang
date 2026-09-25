//! `corpus.lock`: the pinned corpus release and how its assets split into shards.

use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CorpusLock {
    pub repo: String,
    pub tag: String,
    pub pr: PrAsset,
    #[serde(default)]
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct PrAsset {
    pub asset: String,
    pub sha256: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Asset {
    pub name: String,
    pub version: String,
    pub contracts: u64,
    pub bytes: u64,
    pub sha256: String,
}

impl CorpusLock {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read corpus lock {path:?}"))?;
        toml::from_str(&contents).with_context(|| format!("Malformed corpus lock {path:?}"))
    }

    /// Assets of shard `index` out of `count`: every asset goes to the shard with the
    /// fewest bytes so far, largest first, so compressed size (a proxy for source text
    /// and thus for compile time) is balanced without splitting any asset.
    pub fn shard(&self, count: usize, index: usize) -> Vec<&Asset> {
        let mut by_size: Vec<&Asset> = self.assets.iter().collect();
        by_size.sort_by_key(|asset| (std::cmp::Reverse(asset.bytes), asset.name.as_str()));
        let mut loads = vec![0u64; count];
        let mut shards: Vec<Vec<&Asset>> = vec![Vec::new(); count];
        for asset in by_size {
            let lightest = (0..count)
                .min_by_key(|&shard| (loads[shard], shard))
                .expect("at least one shard");
            loads[lightest] += asset.bytes;
            shards[lightest].push(asset);
        }
        let mut shard = std::mem::take(&mut shards[index]);
        shard.sort_by(|a, b| a.name.cmp(&b.name));
        shard
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    fn lock(sizes: &[u64]) -> CorpusLock {
        let mut assets = String::new();
        for (i, bytes) in sizes.iter().enumerate() {
            writeln!(
                assets,
                "[[assets]]\nname = \"a{i}.tar.xz\"\nversion = \"0.8.{i}\"\ncontracts = 1\nbytes = {bytes}\nsha256 = \"x\""
            )
            .unwrap();
        }
        toml::from_str(&format!(
            "repo = \"r\"\ntag = \"t\"\n[pr]\nasset = \"p\"\nsha256 = \"y\"\n{assets}"
        ))
        .unwrap()
    }

    #[test]
    fn every_asset_lands_in_exactly_one_shard() {
        let lock = lock(&[800, 810, 50, 40, 30, 20, 10, 5, 3, 2]);
        let mut seen: Vec<&str> = (0..3)
            .flat_map(|i| lock.shard(3, i))
            .map(|a| a.name.as_str())
            .collect();
        seen.sort_unstable();
        let mut all: Vec<&str> = lock.assets.iter().map(|a| a.name.as_str()).collect();
        all.sort_unstable();
        assert_eq!(seen, all);
    }

    #[test]
    fn the_largest_assets_get_their_own_shard() {
        let lock = lock(&[800, 810, 50, 40, 30, 20, 10, 5, 3, 2]);
        let loads: Vec<u64> = (0..3)
            .map(|i| lock.shard(3, i).iter().map(|a| a.bytes).sum())
            .collect();
        assert_eq!(loads, vec![810, 800, 160]);
    }

    #[test]
    fn the_crate_lock_parses_and_covers_every_version_once() {
        let lock =
            CorpusLock::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus.lock")).unwrap();
        let mut versions: Vec<&str> = lock.assets.iter().map(|a| a.version.as_str()).collect();
        versions.sort_unstable();
        versions.dedup();
        assert_eq!(versions.len(), lock.assets.len());
        assert!(!lock.pr.asset.is_empty());
    }
}
