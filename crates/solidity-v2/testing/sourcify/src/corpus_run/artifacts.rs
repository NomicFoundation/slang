//! Output checks against the solc artifacts a corpus record carries for its target
//! contract: its ABI, and its storage and transient storage layouts.

use ruint::aliases::U256;
use serde_json::Value;
use slang_solidity_v2::abi::{
    StorageLayout, StorageMember, StorageSize, StorageType, StorageTypeKind,
};
use slang_solidity_v2::ast::Definition;
use slang_solidity_v2::compilation::CompilationUnit;
use slang_solidity_v2_common::collections::{Set, SortedMap};

use super::outcome::{Check, Failure};
use crate::corpus::CorpusContract;

/// The deployed contract the artifacts describe: the record's `target_contract` when it
/// has one, else the target file's single concrete contract or library, else the
/// contract solc's storage layout names.
pub fn target_definition(
    unit: &CompilationUnit,
    record: &CorpusContract,
) -> Result<Definition, String> {
    let mut candidates: Vec<(String, Definition)> = Vec::new();
    for definition in unit.all_definitions() {
        let name = match &definition {
            Definition::Contract(contract)
                if contract.get_file_id().as_str() == record.target && !contract.is_abstract() =>
            {
                contract.name().name().to_owned()
            }
            Definition::Library(library) if library.get_file_id().as_str() == record.target => {
                library.name().name().to_owned()
            }
            _ => continue,
        };
        candidates.push((name, definition));
    }
    if let Some(name) = &record.target_contract {
        return match candidates
            .iter()
            .position(|(candidate, _)| candidate == name)
        {
            Some(index) => Ok(candidates.swap_remove(index).1),
            None => Err("target contract not found in the target file".to_owned()),
        };
    }
    match candidates.len() {
        0 => Err("no concrete contract or library in the target file".to_owned()),
        1 => Ok(candidates.pop().expect("one candidate").1),
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
                (Some(index), None) => Ok(candidates.swap_remove(index).1),
                _ => {
                    Err("ambiguous target: the file has several deployable definitions".to_owned())
                }
            }
        }
    }
}

/// Which of solc's two layouts a storage check compares.
#[derive(Clone, Copy)]
pub enum Layout {
    Storage,
    Transient,
}

impl Layout {
    fn artifact_key(self) -> &'static str {
        match self {
            Layout::Storage => "storageLayout",
            Layout::Transient => "transientStorageLayout",
        }
    }

    /// The check for label, slot and offset, and the one for the types table.
    pub fn checks(self) -> (Check, Check) {
        match self {
            Layout::Storage => (Check::StorageLayout, Check::StorageTypes),
            Layout::Transient => (Check::TransientStorageLayout, Check::TransientStorageTypes),
        }
    }

    /// A library has no state variables, so its layouts are empty.
    fn compute(self, target: &Definition) -> Result<StorageLayout, String> {
        let layout = match (target, self) {
            (Definition::Contract(contract), Layout::Storage) => contract.compute_storage_layout(),
            (Definition::Contract(contract), Layout::Transient) => {
                contract.compute_transient_storage_layout()
            }
            _ => Some(StorageLayout::default()),
        };
        layout.ok_or_else(|| "Slang computed no storage layout for the target".to_owned())
    }
}

/// The storage checks for one layout: the first compares each item's label, slot and
/// offset with solc's list; the second walks each item's type through Slang's types
/// table and solc's side by side, comparing label, size, encoding and what the type
/// is made of.
pub fn check_storage_layout(
    target: &Definition,
    artifacts: &Value,
    layout: Layout,
) -> Result<Vec<Failure>, String> {
    let key = layout.artifact_key();
    let (layout_check, types_check) = layout.checks();
    // solc writes `{"storage": [], "types": null}` for a contract without storage, and no
    // `transientStorageLayout` before 0.8.27.
    let solc_items = artifacts
        .pointer(&format!("/{key}/storage"))
        .and_then(Value::as_array)
        .ok_or_else(|| format!("no {key} in the artifacts"))?;
    let slang_layout = layout.compute(target)?;
    let slang_items = slang_layout.items();

    if slang_items.len() != solc_items.len() {
        return Ok(vec![Failure {
            check: layout_check,
            code: "count".to_owned(),
            count: 1,
            message: format!(
                "slang {} items, solc {}",
                slang_items.len(),
                solc_items.len()
            ),
        }]);
    }
    let mut failures = Failures::default();
    let mut types = TypesWalk {
        slang: &slang_layout,
        solc: artifacts.pointer(&format!("/{key}/types")),
        check: types_check,
        visited: Set::default(),
    };
    for (index, (slang, solc)) in slang_items.iter().zip(solc_items).enumerate() {
        let solc_label = solc["label"].as_str().unwrap_or_default();
        let solc_slot = solc["slot"].as_str().unwrap_or_default();
        let solc_offset = solc["offset"].as_u64().unwrap_or_default();
        let mut mismatch = |field: &str, slang_value: String, solc_value: String| {
            failures.add(
                layout_check,
                format!("[*].{field}"),
                format!("item {index} `{solc_label}`: slang {slang_value}, solc {solc_value}"),
            );
        };
        if slang.name() != solc_label {
            mismatch("label", slang.name().to_owned(), solc_label.to_owned());
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
        types.compare(
            slang_layout.storage_type(slang.type_id()),
            solc["type"].as_str().unwrap_or_default(),
            &format!("item {index} `{solc_label}`"),
            &mut failures,
        );
    }
    Ok(failures.into_vec())
}

/// Pairs Slang's storage types with solc's by following both from the same item, since
/// the two key their tables differently (`TypeId` against solc's `t_...` names).
struct TypesWalk<'a> {
    slang: &'a StorageLayout,
    solc: Option<&'a Value>,
    check: Check,
    /// solc type names already compared, so a recursive struct is walked once.
    visited: Set<String>,
}

impl TypesWalk<'_> {
    fn compare(
        &mut self,
        slang: Option<&StorageType>,
        solc_name: &str,
        path: &str,
        failures: &mut Failures,
    ) {
        if !self.visited.insert(solc_name.to_owned()) {
            return;
        }
        let check = self.check;
        let Some(slang) = slang else {
            failures.add(
                check,
                "[*].type.missing".to_owned(),
                format!("{path}: no Slang types entry for solc `{solc_name}`"),
            );
            return;
        };
        let Some(solc) = self.solc.and_then(|types| types.get(solc_name)) else {
            failures.add(
                check,
                "[*].type.missing".to_owned(),
                format!("{path}: no solc types entry `{solc_name}`"),
            );
            return;
        };
        let mut mismatch = |field: &str, slang_value: &str, solc_value: &str| {
            failures.add(
                check,
                format!("[*].type.{field}"),
                format!("{path} `{solc_name}`: slang `{slang_value}`, solc `{solc_value}`"),
            );
        };
        let field = |name: &str| solc[name].as_str().unwrap_or_default();

        if slang.label() != field("label") {
            mismatch("label", slang.label(), field("label"));
        }
        let number_of_bytes = match slang.size() {
            // solc computes the byte count modulo 2^256, so a `uint256[2**255]` gap is 0 bytes.
            StorageSize::Slots(slots) => slots.wrapping_mul(U256::from(32)).to_string(),
            StorageSize::Bytes(bytes) => bytes.to_string(),
        };
        if number_of_bytes != field("numberOfBytes") {
            mismatch("numberOfBytes", &number_of_bytes, field("numberOfBytes"));
        }
        let encoding = match slang.kind() {
            StorageTypeKind::Value
            | StorageTypeKind::FixedSizeArray { .. }
            | StorageTypeKind::Struct { .. } => "inplace",
            StorageTypeKind::Bytes => "bytes",
            StorageTypeKind::DynamicArray { .. } => "dynamic_array",
            StorageTypeKind::Mapping { .. } => "mapping",
        };
        if encoding != field("encoding") {
            mismatch("encoding", encoding, field("encoding"));
            return;
        }

        match slang.kind() {
            StorageTypeKind::Value | StorageTypeKind::Bytes => {}
            StorageTypeKind::DynamicArray { element }
            | StorageTypeKind::FixedSizeArray { element, .. } => {
                let base = field("base").to_owned();
                self.compare(
                    self.slang.storage_type(*element),
                    &base,
                    &format!("{path} base"),
                    failures,
                );
            }
            StorageTypeKind::Mapping { key, value } => {
                let (solc_key, solc_value) = (field("key").to_owned(), field("value").to_owned());
                self.compare(
                    self.slang.storage_type(*key),
                    &solc_key,
                    &format!("{path} key"),
                    failures,
                );
                self.compare(
                    self.slang.storage_type(*value),
                    &solc_value,
                    &format!("{path} value"),
                    failures,
                );
            }
            StorageTypeKind::Struct { members } => {
                self.compare_members(members, solc, solc_name, path, failures);
            }
        }
    }

    fn compare_members(
        &mut self,
        members: &[StorageMember],
        solc: &Value,
        solc_name: &str,
        path: &str,
        failures: &mut Failures,
    ) {
        let check = self.check;
        let solc_members = solc["members"].as_array().map_or(&[][..], Vec::as_slice);
        if members.len() != solc_members.len() {
            failures.add(
                check,
                "[*].type.members".to_owned(),
                format!(
                    "{path} `{solc_name}`: slang {} members, solc {}",
                    members.len(),
                    solc_members.len()
                ),
            );
            return;
        }
        for (slang_member, solc_member) in members.iter().zip(solc_members) {
            let label = solc_member["label"].as_str().unwrap_or_default();
            let position = slang_member.position();
            let slang_fields = [
                ("label", slang_member.name().to_owned()),
                ("slot", position.slot.to_string()),
                ("offset", position.offset.to_string()),
            ];
            for (name, slang_value) in slang_fields {
                let solc_value = match &solc_member[name] {
                    Value::String(text) => text.clone(),
                    other => other.to_string(),
                };
                if slang_value != solc_value {
                    failures.add(
                        check,
                        format!("[*].type.members.{name}"),
                        format!(
                            "{path} `{solc_name}` member `{label}`: slang {slang_value}, solc {solc_value}"
                        ),
                    );
                }
            }
            let member_type = solc_member["type"].as_str().unwrap_or_default().to_owned();
            self.compare(
                self.slang.storage_type(slang_member.type_id()),
                &member_type,
                &format!("{path} member `{label}`"),
                failures,
            );
        }
    }
}

/// Compares the target's ABI JSON with solc's `abi`: entries are paired by type, name
/// and input types, each pair must match field by field, and the pairs must come in
/// solc's order.
pub fn check_abi(target: &Definition, artifacts: &Value) -> Result<Vec<Failure>, String> {
    let solc_entries = artifacts
        .pointer("/abi")
        .and_then(Value::as_array)
        .ok_or_else(|| "no abi in the artifacts".to_owned())?;
    let abi = match target {
        Definition::Contract(contract) => contract.compute_abi(),
        Definition::Library(library) => library.compute_abi(),
        _ => None,
    }
    .ok_or_else(|| "Slang computed no ABI for the target".to_owned())?;
    let slang_json = serde_json::to_value(&abi).map_err(|error| error.to_string())?;
    let mut unpaired: Vec<Option<&Value>> = slang_json
        .as_array()
        .map(|entries| entries.iter().map(Some).collect())
        .unwrap_or_default();

    let mut failures = Failures::default();
    // (position in Slang's list, key) of each paired entry, in solc's order.
    let mut paired: Vec<(usize, String)> = Vec::new();
    for solc_entry in solc_entries {
        let key = entry_key(solc_entry);
        let position = unpaired
            .iter()
            .position(|entry| entry.is_some_and(|entry| entry_key(entry) == key));
        let Some(position) = position else {
            failures.add(
                Check::Abi,
                "missing".to_owned(),
                format!("solc has `{key}`, slang does not"),
            );
            continue;
        };
        let slang_entry = unpaired[position].take().expect("an unpaired entry");
        compare_fields(&key, "", slang_entry, solc_entry, &mut failures);
        paired.push((position, key));
    }
    for entry in unpaired.into_iter().flatten() {
        failures.add(
            Check::Abi,
            "extra".to_owned(),
            format!("slang has `{}`, solc does not", entry_key(entry)),
        );
    }
    if let Some(pair) = paired.windows(2).find(|pair| pair[0].0 > pair[1].0) {
        failures.add(
            Check::Abi,
            "order".to_owned(),
            format!(
                "solc lists `{}` before `{}`, slang after",
                pair[0].1, pair[1].1
            ),
        );
    }
    Ok(failures.into_vec())
}

/// `type name(input types)`, with a tuple spelled by its components, which tells
/// overloads apart.
fn entry_key(entry: &Value) -> String {
    let inputs = entry["inputs"].as_array().map_or(&[][..], Vec::as_slice);
    format!(
        "{} {}({})",
        entry["type"].as_str().unwrap_or_default(),
        entry["name"].as_str().unwrap_or_default(),
        parameter_types(inputs)
    )
}

fn parameter_types(parameters: &[Value]) -> String {
    parameters
        .iter()
        .map(|parameter| {
            let spelled = parameter["type"].as_str().unwrap_or_default();
            match parameter["components"].as_array() {
                Some(components) => format!("{spelled}({})", parameter_types(components)),
                None => spelled.to_owned(),
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Reports every field of an entry or parameter that differs, as `[*].<path>`; the
/// parameter lists are compared parameter by parameter, down through `components`.
fn compare_fields(key: &str, path: &str, slang: &Value, solc: &Value, failures: &mut Failures) {
    let (Some(slang_fields), Some(solc_fields)) = (slang.as_object(), solc.as_object()) else {
        return;
    };
    let mut names: Vec<&String> = slang_fields.keys().chain(solc_fields.keys()).collect();
    names.sort();
    names.dedup();
    for name in names {
        let field = format!("{path}{name}");
        let (slang_value, solc_value) = (&slang[name.as_str()], &solc[name.as_str()]);
        match (slang_value.as_array(), solc_value.as_array()) {
            (Some(slang_list), Some(solc_list)) if slang_list.len() == solc_list.len() => {
                for (slang_item, solc_item) in slang_list.iter().zip(solc_list) {
                    compare_fields(key, &format!("{field}."), slang_item, solc_item, failures);
                }
            }
            _ if slang_value != solc_value => failures.add(
                Check::Abi,
                format!("[*].{field}"),
                format!("`{key}` {field}: slang `{slang_value}`, solc `{solc_value}`"),
            ),
            _ => {}
        }
    }
}

/// A contract's failures, one per (check, code) with a count, as for diagnostics, so a
/// bucket counts contracts rather than mismatching items.
#[derive(Default)]
struct Failures(SortedMap<(Check, String), Failure>);

impl Failures {
    fn add(&mut self, check: Check, code: String, message: String) {
        self.0
            .entry((check, code.clone()))
            .and_modify(|failure| failure.count += 1)
            .or_insert(Failure {
                check,
                code,
                count: 1,
                message,
            });
    }

    fn into_vec(self) -> Vec<Failure> {
        self.0.into_values().collect()
    }
}
