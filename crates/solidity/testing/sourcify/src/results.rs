use std::collections::BTreeMap;
use std::time::Duration;

use indicatif::{FormattedDuration, HumanCount};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShardResults {
    pub source_files: u64,
    pub passed: u64,
    pub failed: u64,
    pub incompatible: u64,
    pub elapsed: Duration,
}

#[derive(Debug)]
pub struct AllResults {
    pub shards: BTreeMap<usize, ShardResults>,
}

impl<'de> Deserialize<'de> for AllResults {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AllResultsVisitor {})
    }
}

struct AllResultsVisitor {}

impl<'de> Visitor<'de> for AllResultsVisitor {
    type Value = AllResults;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a results map")
    }

    fn visit_map<M>(self, mut access: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        use serde::de::Unexpected;

        let mut shards: BTreeMap<usize, ShardResults> = BTreeMap::new();
        while let Some((key, value)) = access.next_entry::<String, String>()? {
            let shard_index = key
                .strip_prefix("__SLANG_SOURCIFY_SHARD_RESULTS__")
                .ok_or(Error::invalid_value(
                    Unexpected::Str(&key),
                    &"a string prefixed with __SLANG_SOURCIFY_SHARD_RESULTS__",
                ))?
                .parse()
                .map_err(|_| {
                    Error::invalid_value(Unexpected::Str(&key), &"a positive shard index")
                })?;
            let shard_results = serde_json::from_str(&value).map_err(|_| {
                Error::invalid_value(
                    Unexpected::Str(&value),
                    &"a JSON string with the shard results",
                )
            })?;
            shards.insert(shard_index, shard_results);
        }

        Ok(AllResults { shards })
    }
}

pub fn display_all_results(all_results: &AllResults) {
    let mut totals = ShardResults::default();
    println!("Shard ID | Source files |       Passed |       Failed | Incompatible | Elapsed");
    println!("----------------------------------------------------------------------------------");
    for (shard_index, shard_results) in &all_results.shards {
        println!(
            "{shard_index:<8} | \
             {source_files:>12} | \
             {passed:>12} | \
             {failed:>12} | \
             {incompatible:>12} | \
             {elapsed}",
            source_files = format!("{}", HumanCount(shard_results.source_files)),
            passed = format!("{}", HumanCount(shard_results.passed)),
            failed = format!("{}", HumanCount(shard_results.failed)),
            incompatible = format!("{}", HumanCount(shard_results.incompatible)),
            elapsed = FormattedDuration(shard_results.elapsed),
        );
        totals.source_files += shard_results.source_files;
        totals.passed += shard_results.passed;
        totals.failed += shard_results.failed;
        totals.incompatible += shard_results.incompatible;
        totals.elapsed += shard_results.elapsed;
    }
    println!("------------------------------------------------------------------------------------------------");
    println!(
        "TOTALS   | \
         {source_files:>12} | \
         {passed:>12} | \
         {failed:>12} | \
         {incompatible:>12} | \
         {elapsed}",
        source_files = format!("{}", HumanCount(totals.source_files)),
        passed = format!("{}", HumanCount(totals.passed)),
        failed = format!("{}", HumanCount(totals.failed)),
        incompatible = format!("{}", HumanCount(totals.incompatible)),
        elapsed = FormattedDuration(totals.elapsed),
    );
}
