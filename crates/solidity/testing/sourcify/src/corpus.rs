//! Corpus snapshots: self-contained sets of Sourcify contracts, one JSON file per
//! contract under `contracts/`, next to a `corpus.json` manifest.
//!
//! Snapshots are produced by the extraction in the corpus repository (see
//! `corpus.lock`) and by hand under `testdata/corpus`. Each record carries what a
//! solc standard-JSON input needs, so solx consumes the same snapshots.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct CorpusManifest {
    pub format_version: u32,
    /// Human-readable provenance: where and how this snapshot was extracted.
    pub description: String,
    pub contract_count: usize,
}

/// A single contract with all of its source files inlined.
#[derive(Serialize, Deserialize)]
pub struct CorpusContract {
    /// Contract address, used as the test case name.
    pub name: String,
    pub chain_id: u64,
    /// Normalized semver of the compiler, without pre-release/build metadata.
    pub version: String,
    /// Virtual path of the compilation entrypoint (the `settings.compilationTarget` key).
    pub target: String,
    /// Name of the deployed contract in `target`, when the snapshot records it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contract: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remappings: Vec<String>,
    /// Virtual path -> source content.
    pub sources: BTreeMap<String, String>,
    /// The contract's original solc `compiler_settings`, verbatim.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub settings: Value,
    /// The EVM target the contract was verified with, as solc spells it
    /// (`cancun`); absent means solc's default for `version`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<String>,
    /// solc's own outputs for the target contract (`abi`, `storageLayout`, `userdoc`,
    /// `devdoc`), the oracle for the output checks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Value>,
}

pub struct Corpus {
    pub manifest: CorpusManifest,
    contract_paths: Vec<PathBuf>,
}

impl Corpus {
    pub fn open(dir: &Path) -> Result<Corpus> {
        let manifest_path = dir.join("corpus.json");
        let manifest: CorpusManifest = serde_json::from_str(
            &fs::read_to_string(&manifest_path)
                .with_context(|| format!("Could not read corpus manifest {manifest_path:?}"))?,
        )?;

        anyhow::ensure!(
            manifest.format_version == FORMAT_VERSION,
            "Unsupported corpus format version {version} (expected {FORMAT_VERSION})",
            version = manifest.format_version
        );

        let mut contract_paths: Vec<_> = fs::read_dir(dir.join("contracts"))?
            .map(|entry| Ok(entry?.path()))
            .collect::<Result<_>>()?;
        contract_paths.sort();

        Ok(Corpus {
            manifest,
            contract_paths,
        })
    }

    /// Contract file paths belonging to the given shard. Contracts are distributed
    /// round-robin over their (sorted) position, so any `shard_count` works.
    pub fn shard(&self, shard_count: usize, shard_index: usize) -> impl Iterator<Item = &PathBuf> {
        self.contract_paths
            .iter()
            .skip(shard_index)
            .step_by(shard_count)
    }
}

pub fn read_contract(path: &Path) -> Result<CorpusContract> {
    serde_json::from_str(
        &fs::read_to_string(path)
            .with_context(|| format!("Could not read corpus contract {path:?}"))?,
    )
    .with_context(|| format!("Malformed corpus contract {path:?}"))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Corpus;
    use crate::corpus_run::testdata_corpus_dir;

    #[test]
    fn shards_partition_the_corpus() {
        let corpus = Corpus::open(&testdata_corpus_dir()).unwrap();

        let all: HashSet<_> = corpus.shard(1, 0).collect();
        let sharded: Vec<_> = (0..2).flat_map(|index| corpus.shard(2, index)).collect();

        assert_eq!(sharded.len(), all.len());
        assert_eq!(sharded.iter().copied().collect::<HashSet<_>>(), all);
        assert_eq!(all.len(), corpus.manifest.contract_count);
    }
}
