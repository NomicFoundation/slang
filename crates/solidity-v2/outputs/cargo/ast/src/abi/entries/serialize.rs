//! The JSON ABI: entries tagged by `type`, structs spelled `tuple` with their `components`,
//! `indexed` on event inputs only, `internalType` rendered from the semantic type, keys in
//! alphabetical order.

use std::fmt;

use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};

use super::{AbiEntries, AbiEntry, AbiMutability, AbiParameter, AbiParameters, ContractAbi};

impl Serialize for ContractAbi {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.entries().serialize(serializer)
    }
}

impl Serialize for AbiEntries<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.iter())
    }
}

impl Serialize for AbiParameters<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.iter())
    }
}

impl Serialize for AbiEntry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AbiEntry::Constructor(constructor) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &constructor.inputs())?;
                map.serialize_entry(
                    "stateMutability",
                    &Mutability(constructor.state_mutability()),
                )?;
                map.serialize_entry("type", "constructor")?;
                map.end()
            }
            AbiEntry::Error(error) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &error.inputs())?;
                map.serialize_entry("name", error.name())?;
                map.serialize_entry("type", "error")?;
                map.end()
            }
            AbiEntry::Event(event) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("anonymous", &event.anonymous())?;
                map.serialize_entry("inputs", &event.inputs())?;
                map.serialize_entry("name", event.name())?;
                map.serialize_entry("type", "event")?;
                map.end()
            }
            AbiEntry::Fallback(fallback) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("stateMutability", &Mutability(fallback.state_mutability()))?;
                map.serialize_entry("type", "fallback")?;
                map.end()
            }
            AbiEntry::Function(function) => {
                let mut map = serializer.serialize_map(Some(5))?;
                map.serialize_entry("inputs", &function.inputs())?;
                map.serialize_entry("name", function.name())?;
                map.serialize_entry("outputs", &function.outputs())?;
                map.serialize_entry("stateMutability", &Mutability(function.state_mutability()))?;
                map.serialize_entry("type", "function")?;
                map.end()
            }
            AbiEntry::Receive(receive) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("stateMutability", &Mutability(receive.state_mutability()))?;
                map.serialize_entry("type", "receive")?;
                map.end()
            }
        }
    }
}

/// A parameter, event input or struct member.
impl Serialize for AbiParameter<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let components = self.components();
        let mut map = serializer.serialize_map(Some(
            3 + usize::from(components.is_some()) + usize::from(self.indexed.is_some()),
        ))?;
        if let Some(components) = components {
            map.serialize_entry("components", &components)?;
        }
        if let Some(indexed) = self.indexed {
            map.serialize_entry("indexed", &indexed)?;
        }
        map.serialize_entry(
            "internalType",
            &format_args!(
                "{}",
                fmt::from_fn(|f| self.semantic.write_type_abi_internal_name(self.type_id, f))
            ),
        )?;
        map.serialize_entry("name", self.name.unwrap_or_default())?;
        map.serialize_entry(
            "type",
            &format_args!(
                "{}",
                fmt::from_fn(|f| self.write_type_name(self.spelling_policy.json(), f))
            ),
        )?;
        map.end()
    }
}

struct Mutability<'a>(&'a AbiMutability);

impl Serialize for Mutability<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self.0 {
            AbiMutability::Pure => "pure",
            AbiMutability::View => "view",
            AbiMutability::NonPayable => "nonpayable",
            AbiMutability::Payable => "payable",
        })
    }
}
