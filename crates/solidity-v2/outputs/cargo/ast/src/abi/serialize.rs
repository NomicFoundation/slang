//! The JSON-ABI spelling of the ABI, as solc emits it: entries tagged by `type`, structs spelled
//! `tuple` with their `components`, `indexed` on event inputs only, `internalType` rendered from
//! the semantic type, and keys in alphabetical order. `serde_json::to_value(abi.json())` is the
//! `abi` field of solc's standard JSON.

use std::sync::Arc;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::{AbiEntry, AbiMutability, AbiParameter, AbiType, ContractAbi, TupleComponent};
use crate::ast::{Definition as AstDefinition, Type as AstType};

/// A contract's entries as solc's JSON ABI; see [`ContractAbi::json`].
pub struct JsonAbi<'a>(pub(crate) &'a ContractAbi);

impl Serialize for JsonAbi<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let semantic = &self.0.semantic;
        let mut seq = serializer.serialize_seq(Some(self.0.entries.len()))?;
        for entry in &self.0.entries {
            seq.serialize_element(&Entry { entry, semantic })?;
        }
        seq.end()
    }
}

struct Entry<'a> {
    entry: &'a AbiEntry,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for Entry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let parameters = |parameters| ParameterList {
            parameters,
            indexed: false,
            semantic: self.semantic,
        };
        match self.entry {
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
                map.serialize_entry(
                    "inputs",
                    &ParameterList {
                        parameters: event.inputs(),
                        indexed: true,
                        semantic: self.semantic,
                    },
                )?;
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
                map.serialize_entry("inputs", &parameters(function.inputs()))?;
                map.serialize_entry("name", function.name())?;
                map.serialize_entry("outputs", &parameters(function.outputs()))?;
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

/// A parameter list; `indexed` is written only for event inputs, as in solc.
struct ParameterList<'a> {
    parameters: &'a [AbiParameter],
    indexed: bool,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for ParameterList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.parameters.len()))?;
        for parameter in self.parameters {
            seq.serialize_element(&Parameter {
                name: parameter.name().unwrap_or_default(),
                abi_type: parameter.abi_type(),
                type_id: parameter.type_id(),
                indexed: self.indexed.then(|| parameter.indexed()),
                semantic: self.semantic,
            })?;
        }
        seq.end()
    }
}

/// `{components?, indexed?, internalType, name, type}`: `components` only when the type is or
/// contains a struct, `indexed` only for event inputs.
struct Parameter<'a> {
    name: &'a str,
    abi_type: &'a AbiType,
    type_id: TypeId,
    indexed: Option<bool>,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for Parameter<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let components = self.abi_type.tuple_components();
        let mut map = serializer.serialize_map(Some(
            3 + usize::from(components.is_some()) + usize::from(self.indexed.is_some()),
        ))?;
        if let Some(components) = components {
            map.serialize_entry(
                "components",
                &ComponentList {
                    components,
                    type_id: self.type_id,
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
        map.serialize_entry("type", &self.abi_type.json_name())?;
        map.end()
    }
}

/// The members of the struct behind a `tuple`, paired with their semantic types for `internalType`.
struct ComponentList<'a> {
    components: &'a [TupleComponent],
    type_id: TypeId,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for ComponentList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let member_type_ids = struct_member_type_ids(self.type_id, self.semantic);
        assert_eq!(
            member_type_ids.len(),
            self.components.len(),
            "the tuple components are the struct's members"
        );
        let mut seq = serializer.serialize_seq(Some(self.components.len()))?;
        for (component, member_type_id) in self.components.iter().zip(member_type_ids) {
            seq.serialize_element(&Parameter {
                name: component.name(),
                abi_type: component.abi_type(),
                type_id: member_type_id,
                indexed: None,
                semantic: self.semantic,
            })?;
        }
        seq.end()
    }
}

/// The member types of the struct a `tuple` (or `tuple[]`, `tuple[N]`) stands for, in declaration
/// order — the same walk [`AbiType`]'s conversion made to build the components.
fn struct_member_type_ids(type_id: TypeId, semantic: &Arc<SemanticContext>) -> Vec<TypeId> {
    let mut ast_type = AstType::create(type_id, semantic);
    loop {
        ast_type = match ast_type {
            AstType::Array(array) => array.element_type(),
            AstType::FixedSizeArray(array) => array.element_type(),
            AstType::ArraySlice(slice) => slice.array_type(),
            AstType::Struct(struct_type) => {
                let AstDefinition::Struct(definition) = struct_type.definition() else {
                    unreachable!("a struct type resolves to a struct definition");
                };
                return definition
                    .members()
                    .iter()
                    .map(|member| {
                        semantic
                            .binder()
                            .node_typing(member.node_id())
                            .as_type_id()
                            .expect("a struct member that reached the ABI is typed")
                    })
                    .collect();
            }
            _ => unreachable!("only a struct, or an array of structs, has tuple components"),
        };
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
