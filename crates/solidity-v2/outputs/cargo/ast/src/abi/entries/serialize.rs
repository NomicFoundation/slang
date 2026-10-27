//! The JSON ABI: entries tagged by `type`, structs spelled `tuple` with their `components`,
//! `indexed` on event inputs only, `internalType` rendered from the semantic type, keys in
//! alphabetical order.

use std::fmt;

use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};

use super::{AbiEntry, AbiMutability, AbiParameter, ContractAbi};

impl Serialize for ContractAbi {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.entries())
    }
}

impl Serialize for AbiEntry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AbiEntry::Constructor(constructor) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &Seq(constructor.inputs()))?;
                map.serialize_entry(
                    "stateMutability",
                    &Mutability(constructor.state_mutability()),
                )?;
                map.serialize_entry("type", "constructor")?;
                map.end()
            }
            AbiEntry::Error(error) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &Seq(error.inputs()))?;
                map.serialize_entry("name", error.name())?;
                map.serialize_entry("type", "error")?;
                map.end()
            }
            AbiEntry::Event(event) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("anonymous", &event.anonymous())?;
                map.serialize_entry("inputs", &Seq(event.inputs()))?;
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
                map.serialize_entry("inputs", &Seq(function.inputs()))?;
                map.serialize_entry("name", function.name())?;
                map.serialize_entry("outputs", &Seq(function.outputs()))?;
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
            map.serialize_entry("components", &Seq(components))?;
        }
        if let Some(indexed) = self.indexed {
            map.serialize_entry("indexed", &indexed)?;
        }
        map.serialize_entry(
            "internalType",
            &written(|f| self.semantic.write_type_abi_internal_name(self.type_id, f)),
        )?;
        map.serialize_entry("name", self.name.unwrap_or_default())?;
        map.serialize_entry(
            "type",
            &written(|f| self.write_type_name(self.spelling_policy.json(), f)),
        )?;
        map.end()
    }
}

/// Serializes as the sequence of the views an iterator yields.
struct Seq<I>(I);

impl<I> Serialize for Seq<I>
where
    I: Iterator + Clone,
    I::Item: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.clone())
    }
}

/// Serializes as the string a closure writes, streaming it into the output instead of building
/// it first.
struct Written<F>(F);

fn written<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>(write: F) -> Written<F> {
    Written(write)
}

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for Written<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> Serialize for Written<F> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
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
