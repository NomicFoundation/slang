//! The JSON ABI: entries tagged by `type`, structs spelled `tuple` with their `components`,
//! `indexed` on event inputs only, `internalType` rendered from the semantic type, keys in
//! alphabetical order.

use std::fmt;
use std::sync::Arc;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::types::{AbiShape, abi_shape};
use crate::abi::{AbiEntry, AbiMutability, AbiParameter, ContractAbi, TypeSpelling};
use crate::ast::{StructDefinition, Type as AstType};

impl Serialize for ContractAbi {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.entries.len()))?;
        for entry in &self.entries {
            seq.serialize_element(entry)?;
        }
        seq.end()
    }
}

impl Serialize for AbiEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let parameters = |parameters| ParameterList {
            parameters,
            spelling: TypeSpelling::Canonical,
        };
        match self {
            AbiEntry::Constructor(constructor) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &parameters(constructor.inputs()))?;
                map.serialize_entry(
                    "stateMutability",
                    &Mutability(constructor.state_mutability()),
                )?;
                map.serialize_entry("type", "constructor")?;
                map.end()
            }
            AbiEntry::Error(error) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("inputs", &parameters(error.inputs()))?;
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
                let function_parameters = |parameters| ParameterList {
                    parameters,
                    spelling: function.type_spelling,
                };
                let mut map = serializer.serialize_map(Some(5))?;
                map.serialize_entry("inputs", &function_parameters(function.inputs()))?;
                map.serialize_entry("name", function.name())?;
                map.serialize_entry("outputs", &function_parameters(function.outputs()))?;
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

struct ParameterList<'a> {
    parameters: &'a [AbiParameter],
    spelling: TypeSpelling,
}

impl Serialize for ParameterList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.parameters.len()))?;
        for parameter in self.parameters {
            seq.serialize_element(&Parameter {
                name: parameter.name().unwrap_or_default(),
                type_id: parameter.type_id,
                indexed: None,
                spelling: self.spelling,
                semantic: &parameter.semantic,
            })?;
        }
        seq.end()
    }
}

/// An event's inputs, the only parameters that carry `indexed`.
struct EventInputs<'a>(&'a [AbiParameter]);

impl Serialize for EventInputs<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for parameter in self.0 {
            seq.serialize_element(&Parameter {
                name: parameter.name().unwrap_or_default(),
                type_id: parameter.type_id,
                indexed: Some(parameter.indexed()),
                spelling: TypeSpelling::Canonical,
                semantic: &parameter.semantic,
            })?;
        }
        seq.end()
    }
}

/// A parameter, event input or struct member.
struct Parameter<'a> {
    name: &'a str,
    type_id: TypeId,
    indexed: Option<bool>,
    spelling: TypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for Parameter<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = AstType::create(self.type_id, self.semantic);
        let definition = struct_behind(&value);
        let mut map = serializer.serialize_map(Some(
            3 + usize::from(definition.is_some()) + usize::from(self.indexed.is_some()),
        ))?;
        if let Some(definition) = definition {
            map.serialize_entry(
                "components",
                &ComponentList {
                    definition,
                    spelling: self.spelling,
                    semantic: self.semantic,
                },
            )?;
        }
        if let Some(indexed) = self.indexed {
            map.serialize_entry("indexed", &indexed)?;
        }
        map.serialize_entry(
            "internalType",
            &self.semantic.type_abi_internal_name(self.type_id),
        )?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry(
            "type",
            &JsonType {
                value: &value,
                spelling: self.spelling,
                semantic: self.semantic,
            },
        )?;
        map.end()
    }
}

/// The members of the struct behind a `tuple`, in declaration order.
struct ComponentList<'a> {
    definition: StructDefinition,
    spelling: TypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for ComponentList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let members = &self.definition.ir_node.members;
        let mut seq = serializer.serialize_seq(Some(members.len()))?;
        for member in members.iter() {
            seq.serialize_element(&Parameter {
                name: member.name.unparse(),
                type_id: self
                    .semantic
                    .binder()
                    .node_typing(member.id())
                    .as_type_id()
                    .expect("a struct member in the ABI is typed"),
                indexed: None,
                spelling: self.spelling,
                semantic: self.semantic,
            })?;
        }
        seq.end()
    }
}

/// The JSON-ABI `type` string: a struct is `tuple`, `tuple[]` or `tuple[N]`, everything else its
/// ABI type, with enums, contracts and interfaces spelled as [`TypeSpelling`] says.
struct JsonType<'a> {
    value: &'a AstType,
    spelling: TypeSpelling,
    semantic: &'a SemanticContext,
}

impl fmt::Display for JsonType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.spelling, TypeSpelling::ByName) {
            let definition = match self.value {
                AstType::Contract(contract) => Some(contract.definition()),
                AstType::Enum(enum_type) => Some(enum_type.definition()),
                AstType::Interface(interface) => Some(interface.definition()),
                _ => None,
            };
            if let Some(definition) = definition {
                return f.write_str(
                    &self
                        .semantic
                        .definition_canonical_name(definition.node_id()),
                );
            }
        }
        // The parameter was checked to have an ABI representation when it was built.
        match abi_shape(self.value).expect("a parameter in the ABI has an ABI type") {
            AbiShape::Scalar(scalar) => scalar.fmt(f),
            AbiShape::Array(element) => write!(f, "{}[]", self.of(&element)),
            AbiShape::FixedSizeArray(element, size) => write!(f, "{}[{size}]", self.of(&element)),
            AbiShape::Struct(_) => f.write_str("tuple"),
        }
    }
}

impl JsonType<'_> {
    fn of<'b>(&'b self, value: &'b AstType) -> JsonType<'b> {
        JsonType { value, ..*self }
    }
}

impl Serialize for JsonType<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// The struct behind a `tuple` type, through any arrays.
fn struct_behind(value: &AstType) -> Option<StructDefinition> {
    match abi_shape(value)? {
        AbiShape::Array(element) | AbiShape::FixedSizeArray(element, _) => struct_behind(&element),
        AbiShape::Struct(definition) => Some(definition),
        AbiShape::Scalar(_) => None,
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
