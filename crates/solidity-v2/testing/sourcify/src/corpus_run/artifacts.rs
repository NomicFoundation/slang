//! Output checks against the solc artifacts a corpus record carries for its target
//! contract: the ABI, and the storage layout.

use std::collections::BTreeMap;

use serde_json::Value;
use slang_solidity_v2::abi::ContractAbi;
use slang_solidity_v2::ast::Definition;
use slang_solidity_v2::compilation::CompilationUnit;

use super::outcome::{Check, Failure};
use crate::corpus::CorpusContract;

/// The ABI of the deployed contract the artifacts describe: the record's
/// `target_contract` when it has one, else the target file's single concrete contract
/// or library, else the contract solc's storage layout names.
pub fn target_abi(unit: &CompilationUnit, record: &CorpusContract) -> Result<ContractAbi, String> {
    let mut candidates: Vec<(String, Option<ContractAbi>)> = Vec::new();
    for definition in unit.all_definitions() {
        match &definition {
            Definition::Contract(contract)
                if contract.get_file_id().as_str() == record.target && !contract.is_abstract() =>
            {
                candidates.push((contract.name().name().to_owned(), contract.compute_abi()));
            }
            Definition::Library(library) if library.get_file_id().as_str() == record.target => {
                candidates.push((library.name().name().to_owned(), library.compute_abi()));
            }
            _ => {}
        }
    }
    if let Some(name) = &record.target_contract {
        return match candidates
            .iter()
            .position(|(candidate, _)| candidate == name)
        {
            Some(index) => candidates
                .swap_remove(index)
                .1
                .ok_or_else(|| "Slang computed no ABI for the target".to_owned()),
            None => Err("target contract not found in the target file".to_owned()),
        };
    }
    let chosen = match candidates.len() {
        0 => return Err("no concrete contract or library in the target file".to_owned()),
        1 => candidates.pop().expect("one candidate"),
        _ => {
            let named: Vec<&str> = record
                .artifacts
                .as_ref()
                .and_then(|artifacts| artifacts.pointer("/storageLayout/storage"))
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item["contract"].as_str())
                        .filter_map(|qualified| qualified.rsplit_once(':').map(|(_, name)| name))
                        .collect()
                })
                .unwrap_or_default();
            let mut hinted = candidates
                .iter()
                .enumerate()
                .filter(|(_, (name, _))| named.contains(&name.as_str()))
                .map(|(index, _)| index);
            match (hinted.next(), hinted.next()) {
                (Some(index), None) => candidates.swap_remove(index),
                _ => {
                    return Err(
                        "ambiguous target: the file has several deployable definitions".to_owned(),
                    );
                }
            }
        }
    };
    chosen
        .1
        .ok_or_else(|| "Slang computed no ABI for the target".to_owned())
}

/// The storage layout checks: `storage_layout` compares each slot's label, slot and
/// offset with solc's `storageLayout.storage`; `storage_types` compares the type
/// spelling with solc's label for that type.
pub fn check_storage_layout(abi: &ContractAbi, artifacts: &Value) -> Result<Vec<Failure>, String> {
    // solc writes `{"storage": [], "types": null}` for a contract without storage.
    let solc_items = artifacts
        .pointer("/storageLayout/storage")
        .and_then(Value::as_array)
        .ok_or_else(|| "no storageLayout in the artifacts".to_owned())?;
    let solc_types = artifacts.pointer("/storageLayout/types");
    let slang_items = abi.storage_layout();

    if slang_items.len() != solc_items.len() {
        return Ok(vec![Failure {
            check: Check::StorageLayout,
            code: "count".to_owned(),
            count: 1,
            message: format!(
                "slang {} items, solc {}",
                slang_items.len(),
                solc_items.len()
            ),
        }]);
    }
    // One failure per code, as for diagnostics, so a bucket counts contracts, not items.
    let mut failures: BTreeMap<(Check, String), Failure> = BTreeMap::new();
    let mut add = |check: Check, code: String, message: String| {
        failures
            .entry((check, code.clone()))
            .and_modify(|failure| failure.count += 1)
            .or_insert(Failure {
                check,
                code,
                count: 1,
                message,
            });
    };
    for (index, (slang, solc)) in slang_items.iter().zip(solc_items).enumerate() {
        let solc_label = solc["label"].as_str().unwrap_or_default();
        let solc_slot = solc["slot"].as_str().unwrap_or_default();
        let solc_offset = solc["offset"].as_u64().unwrap_or_default();
        let mut mismatch = |field: &str, slang_value: String, solc_value: String| {
            add(
                Check::StorageLayout,
                format!("[*].{field}"),
                format!("item {index} `{solc_label}`: slang {slang_value}, solc {solc_value}"),
            );
        };
        if slang.label() != solc_label {
            mismatch("label", slang.label().to_owned(), solc_label.to_owned());
        }
        if slang.slot().to_string() != solc_slot {
            mismatch("slot", slang.slot().to_string(), solc_slot.to_owned());
        }
        if slang.offset() as u64 != solc_offset {
            mismatch(
                "offset",
                slang.offset().to_string(),
                solc_offset.to_string(),
            );
        }
        let solc_type = solc["type"].as_str().unwrap_or_default();
        let solc_type_label = solc_types
            .and_then(|types| types[solc_type]["label"].as_str())
            .unwrap_or(solc_type);
        if slang.type_name() != solc_type_label {
            add(
                Check::StorageTypes,
                "[*].type".to_owned(),
                format!(
                    "item {index} `{solc_label}`: slang `{}`, solc `{solc_type_label}`",
                    slang.type_name()
                ),
            );
        }
    }
    Ok(failures.into_values().collect())
}
