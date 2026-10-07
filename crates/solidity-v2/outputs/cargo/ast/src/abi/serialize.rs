//! The JSON ABI: entries tagged by `type`, structs spelled `tuple` with their `components`,
//! `indexed` on event inputs only, `internalType` rendered from the semantic type, keys in
//! alphabetical order.

use std::fmt;
use std::sync::Arc;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder::Definition;
use slang_solidity_v2_semantic::context::{AbiNameError, AbiTypeSpelling, SemanticContext};
use slang_solidity_v2_semantic::types::{
    ArraySliceType, ArrayType, FixedSizeArrayType, StructType, Type, TypeId,
};

use crate::abi::{AbiEntry, AbiMutability, AbiParameter, ContractAbi};

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
            spelling: AbiTypeSpelling::Json,
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
    spelling: AbiTypeSpelling,
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
                spelling: AbiTypeSpelling::Json,
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
    spelling: AbiTypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for Parameter<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let struct_id = struct_behind(self.semantic, self.type_id);
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
            &written(|f| self.semantic.write_type_abi_internal_name(self.type_id, f)),
        )?;
        map.serialize_entry("name", self.name)?;
        map.serialize_entry(
            "type",
            &written(|f| {
                self.semantic
                    .write_type_abi_name(self.type_id, self.spelling, f)
                    .map_err(|error| match error {
                        AbiNameError::Write(error) => error,
                        // The parameter was checked to have an ABI type when it was built.
                        error => unreachable!("a parameter in the ABI has an ABI type: {error}"),
                    })
            }),
        )?;
        map.end()
    }
}

/// The members of the struct behind a `tuple`, in declaration order.
struct ComponentList<'a> {
    struct_id: NodeId,
    spelling: AbiTypeSpelling,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for ComponentList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some(Definition::Struct(definition)) =
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

/// The struct behind a `tuple` type, through any arrays.
fn struct_behind(semantic: &SemanticContext, type_id: TypeId) -> Option<NodeId> {
    match semantic.types().get_type_by_id(type_id) {
        Type::Array(ArrayType { element_type, .. })
        | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
            struct_behind(semantic, *element_type)
        }
        Type::ArraySlice(ArraySliceType { array_type_id }) => {
            struct_behind(semantic, *array_type_id)
        }
        Type::Struct(StructType { definition_id, .. }) => Some(*definition_id),
        _ => None,
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
