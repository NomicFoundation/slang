//! The JSON ABI: entries tagged by `type`, structs spelled `tuple` with their `components`,
//! `indexed` on event inputs only, `internalType` rendered from the semantic type, keys in
//! alphabetical order.

use std::fmt;
use std::sync::Arc;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{self, TypeId};

use crate::abi::types::type_as_abi_type;
use crate::abi::{AbiEntry, AbiMutability, AbiParameter, ContractAbi, TypeSpelling};

/// A contract's entries as the JSON ABI; see [`ContractAbi::json`].
pub struct JsonAbi<'a>(pub(crate) &'a ContractAbi);

impl Serialize for JsonAbi<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.0.entries.len()))?;
        for entry in &self.0.entries {
            seq.serialize_element(&Entry(entry))?;
        }
        seq.end()
    }
}

struct Entry<'a>(&'a AbiEntry);

impl Serialize for Entry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let parameters = |parameters| ParameterList {
            parameters,
            spelling: TypeSpelling::Canonical,
        };
        match self.0 {
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
        let struct_id = struct_behind(self.type_id, self.semantic);
        let mut map = serializer.serialize_map(Some(
            3 + usize::from(struct_id.is_some()) + usize::from(self.indexed.is_some()),
        ))?;
        if let Some(struct_id) = struct_id {
            map.serialize_entry(
                "components",
                &ComponentList {
                    struct_id,
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
                type_id: self.type_id,
                spelling: self.spelling,
                semantic: self.semantic,
            },
        )?;
        map.end()
    }
}

/// The members of the struct behind a `tuple`, in declaration order.
struct ComponentList<'a> {
    struct_id: NodeId,
    spelling: TypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for ComponentList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some(binder::Definition::Struct(definition)) =
            self.semantic.binder().find_definition_by_id(self.struct_id)
        else {
            unreachable!("a struct type resolves to a struct definition");
        };
        let members = &definition.ir_node.members;
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
/// canonical name, with enums, contracts and interfaces spelled as [`TypeSpelling`] says.
/// The parameter was checked to have an ABI representation when it was built, so the type is never
/// declined here.
struct JsonType<'a> {
    type_id: TypeId,
    spelling: TypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl JsonType<'_> {
    fn of(&self, type_id: TypeId) -> Self {
        JsonType { type_id, ..*self }
    }
}

impl fmt::Display for JsonType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.semantic.types().get_type_by_id(self.type_id) {
            types::Type::Array(types::ArrayType { element_type, .. }) => {
                write!(f, "{}[]", self.of(*element_type))
            }
            types::Type::FixedSizeArray(types::FixedSizeArrayType {
                element_type, size, ..
            }) => write!(f, "{}[{size}]", self.of(*element_type)),
            types::Type::ArraySlice(types::ArraySliceType { array_type_id }) => {
                self.of(*array_type_id).fmt(f)
            }
            types::Type::UserDefinedValue(types::UserDefinedValueType { definition_id }) => {
                let Some(binder::Definition::UserDefinedValueType(definition)) =
                    self.semantic.binder().find_definition_by_id(*definition_id)
                else {
                    unreachable!("a user-defined value type resolves to its definition");
                };
                let target_type_id = definition
                    .target_type_id
                    .expect("a user-defined value type in the ABI has a resolved underlying type");
                self.of(target_type_id).fmt(f)
            }
            types::Type::Struct(_) => f.write_str("tuple"),
            types::Type::Contract(_) | types::Type::Enum(_) | types::Type::Interface(_)
                if matches!(self.spelling, TypeSpelling::ByName) =>
            {
                f.write_str(&self.semantic.type_internal_name(self.type_id))
            }
            _ => type_as_abi_type(self.semantic, self.type_id)
                .expect("a scalar in the ABI has an ABI type")
                .fmt(f),
        }
    }
}

impl Serialize for JsonType<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// The struct behind a `tuple` type, through any arrays.
fn struct_behind(type_id: TypeId, semantic: &SemanticContext) -> Option<NodeId> {
    match semantic.types().get_type_by_id(type_id) {
        types::Type::Array(types::ArrayType { element_type, .. })
        | types::Type::FixedSizeArray(types::FixedSizeArrayType { element_type, .. }) => {
            struct_behind(*element_type, semantic)
        }
        types::Type::ArraySlice(types::ArraySliceType { array_type_id }) => {
            struct_behind(*array_type_id, semantic)
        }
        types::Type::Struct(types::StructType { definition_id, .. }) => Some(*definition_id),
        _ => None,
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
