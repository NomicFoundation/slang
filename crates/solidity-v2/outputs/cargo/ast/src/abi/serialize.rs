//! The JSON-ABI spelling of the ABI, as solc emits it: entries tagged by `type`, structs spelled
//! `tuple` with their `components`, `indexed` on event inputs only, and keys in alphabetical
//! order. `serde_json::to_value(abi.entries())` is the `abi` field of solc's standard JSON.

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

use crate::abi::{AbiEntry, AbiMutability, AbiParameter, AbiType, TupleComponent};

impl Serialize for AbiEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AbiEntry::Constructor(constructor) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &Parameters(constructor.inputs()))?;
                map.serialize_entry(
                    "stateMutability",
                    &Mutability(constructor.state_mutability()),
                )?;
                map.serialize_entry("type", "constructor")?;
                map.end()
            }
            AbiEntry::Error(error) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &Parameters(error.inputs()))?;
                map.serialize_entry("name", error.name())?;
                map.serialize_entry("type", "error")?;
                map.end()
            }
            AbiEntry::Event(event) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("anonymous", &event.anonymous())?;
                map.serialize_entry("inputs", &EventInputs(event.inputs()))?;
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
                map.serialize_entry("inputs", &Parameters(function.inputs()))?;
                map.serialize_entry("name", function.name())?;
                map.serialize_entry("outputs", &Parameters(function.outputs()))?;
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

/// A parameter outside an event: no `indexed` key, as in solc.
impl Serialize for AbiParameter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_parameter(
            serializer,
            self.name().unwrap_or_default(),
            self.abi_type(),
            self.internal_type(),
            None,
        )
    }
}

impl Serialize for TupleComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_parameter(
            serializer,
            self.name(),
            self.abi_type(),
            self.internal_type(),
            None,
        )
    }
}

struct Parameters<'a>(&'a [AbiParameter]);

impl Serialize for Parameters<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0)
    }
}

struct EventInputs<'a>(&'a [AbiParameter]);

impl Serialize for EventInputs<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for input in self.0 {
            seq.serialize_element(&EventInput(input))?;
        }
        seq.end()
    }
}

struct EventInput<'a>(&'a AbiParameter);

impl Serialize for EventInput<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_parameter(
            serializer,
            self.0.name().unwrap_or_default(),
            self.0.abi_type(),
            self.0.internal_type(),
            Some(self.0.indexed()),
        )
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

/// `{components?, indexed?, internalType, name, type}`: `components` only when the type is or
/// contains a struct, `indexed` only for event inputs.
fn serialize_parameter<S: Serializer>(
    serializer: S,
    name: &str,
    abi_type: &AbiType,
    internal_type: &str,
    indexed: Option<bool>,
) -> Result<S::Ok, S::Error> {
    let components = abi_type.tuple_components();
    let mut map = serializer.serialize_map(Some(
        3 + usize::from(components.is_some()) + usize::from(indexed.is_some()),
    ))?;
    if let Some(components) = components {
        map.serialize_entry("components", components)?;
    }
    if let Some(indexed) = indexed {
        map.serialize_entry("indexed", &indexed)?;
    }
    map.serialize_entry("internalType", internal_type)?;
    map.serialize_entry("name", name)?;
    map.serialize_entry("type", &abi_type.json_name())?;
    map.end()
}

impl AbiType {
    /// The JSON-ABI `type` string: a struct is `tuple`, an array of structs `tuple[]` or
    /// `tuple[N]`, everything else its canonical name.
    fn json_name(&self) -> String {
        match self {
            AbiType::Array { element } => format!("{}[]", element.json_name()),
            AbiType::FixedSizeArray { element, size } => {
                format!("{}[{size}]", element.json_name())
            }
            AbiType::Tuple(_) => "tuple".to_string(),
            scalar => scalar.to_string(),
        }
    }

    /// The struct members behind a `tuple` spelling, through any array suffixes.
    fn tuple_components(&self) -> Option<&[TupleComponent]> {
        match self {
            AbiType::Array { element } | AbiType::FixedSizeArray { element, .. } => {
                element.tuple_components()
            }
            AbiType::Tuple(components) => Some(components),
            _ => None,
        }
    }
}
