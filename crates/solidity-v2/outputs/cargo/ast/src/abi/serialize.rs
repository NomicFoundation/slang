//! The JSON-ABI spelling of the ABI, as solc emits it: entries tagged by `type`, structs spelled
//! `tuple` with their `components`, `indexed` on event inputs only, `internalType` rendered from
//! the semantic type, keys in alphabetical order, and overloads in ascending selector order.
//! `serde_json::to_value(abi.json())` is the `abi` field of solc's standard JSON.

use std::fmt::{self, Write as _};
use std::sync::Arc;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{self, TypeId};

use crate::abi::types::type_as_abi_type;
use crate::abi::{AbiEntry, AbiFunction, AbiMutability, AbiParameter, ContractAbi};
use crate::ast::Definition as AstDefinition;

/// A contract's entries as solc's JSON ABI; see [`ContractAbi::json`].
pub struct JsonAbi<'a>(pub(crate) &'a ContractAbi);

impl Serialize for JsonAbi<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let library = matches!(
            AstDefinition::try_create(self.0.node_id, &self.0.semantic),
            Some(AstDefinition::Library(_))
        );
        let mut seq = serializer.serialize_seq(Some(self.0.entries.len()))?;
        // The entries are sorted by kind and name, so overloads are adjacent. solc lists them in
        // ascending selector order, which is where its interface function map puts them.
        for run in self.0.entries.chunk_by(same_function_name) {
            if run.len() == 1 {
                seq.serialize_element(&Entry {
                    entry: &run[0],
                    library,
                })?;
                continue;
            }
            let mut overloads: Vec<&AbiEntry> = run.iter().collect();
            overloads.sort_by_cached_key(|entry| match entry {
                AbiEntry::Function(function) => selector(function),
                _ => unreachable!("only functions share a name"),
            });
            for entry in overloads {
                seq.serialize_element(&Entry { entry, library })?;
            }
        }
        seq.end()
    }
}

fn same_function_name(this: &AbiEntry, other: &AbiEntry) -> bool {
    match (this, other) {
        (AbiEntry::Function(this), AbiEntry::Function(other)) => this.name() == other.name(),
        _ => false,
    }
}

/// The selector of the canonical signature, streamed into keccak without building the signature
/// string. Library members hash the library form instead (`FunctionDefinition::compute_selector`),
/// so this only orders overloads.
fn selector(function: &AbiFunction) -> u32 {
    struct Hasher(Keccak256);
    impl fmt::Write for Hasher {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.update(s.as_bytes());
            Ok(())
        }
    }
    let mut hasher = Hasher(Keccak256::new());
    write!(hasher, "{}(", function.name()).expect("hashing cannot fail");
    for (index, input) in function.inputs().iter().enumerate() {
        if index > 0 {
            hasher.write_str(",").expect("hashing cannot fail");
        }
        write!(hasher, "{}", input.abi_type()).expect("hashing cannot fail");
    }
    hasher.write_str(")").expect("hashing cannot fail");
    let hash: [u8; 32] = hasher.0.finalize().into();
    u32::from_be_bytes(hash[0..4].try_into().unwrap())
}

struct Entry<'a> {
    entry: &'a AbiEntry,
    library: bool,
}

impl Serialize for Entry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let parameters = |parameters| ParameterList {
            parameters,
            indexed: false,
            library: false,
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
                        library: false,
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
                // Only a library's functions spell enums, contracts and interfaces by name; its
                // errors and events use the canonical types.
                let parameters = |parameters| ParameterList {
                    parameters,
                    indexed: false,
                    library: self.library,
                };
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
    library: bool,
}

impl Serialize for ParameterList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.parameters.len()))?;
        for parameter in self.parameters {
            seq.serialize_element(&Parameter {
                name: parameter.name().unwrap_or_default(),
                type_id: parameter.type_id,
                indexed: self.indexed.then(|| parameter.indexed()),
                library: self.library,
                semantic: &parameter.semantic,
            })?;
        }
        seq.end()
    }
}

/// `{components?, indexed?, internalType, name, type}`: `components` only when the type is or
/// contains a struct, `indexed` only for event inputs. Struct members are parameters too, minus
/// `indexed`.
struct Parameter<'a> {
    name: &'a str,
    type_id: TypeId,
    indexed: Option<bool>,
    library: bool,
    semantic: &'a Arc<SemanticContext>,
}

impl Serialize for Parameter<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (json_type, struct_id) = json_type(self.type_id, self.semantic, self.library);
        let mut map = serializer.serialize_map(Some(
            3 + usize::from(struct_id.is_some()) + usize::from(self.indexed.is_some()),
        ))?;
        if let Some(struct_id) = struct_id {
            map.serialize_entry(
                "components",
                &ComponentList {
                    struct_id,
                    library: self.library,
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
        map.serialize_entry("type", &json_type)?;
        map.end()
    }
}

/// The members of the struct behind a `tuple`, in declaration order.
struct ComponentList<'a> {
    struct_id: NodeId,
    library: bool,
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
                library: self.library,
                semantic: self.semantic,
            })?;
        }
        seq.end()
    }
}

/// The JSON-ABI `type` string and, for a struct or an array of structs, the struct: a struct is
/// `tuple`, `tuple[]` or `tuple[N]`, everything else its canonical name, except that a library
/// function spells enums, contracts and interfaces by name. The parameter was checked to have an
/// ABI representation when it was built, so the type is never declined here.
fn json_type(
    type_id: TypeId,
    semantic: &Arc<SemanticContext>,
    library: bool,
) -> (String, Option<NodeId>) {
    match semantic.types().get_type_by_id(type_id) {
        types::Type::Array(types::ArrayType { element_type, .. }) => {
            let (element, struct_id) = json_type(*element_type, semantic, library);
            (format!("{element}[]"), struct_id)
        }
        types::Type::FixedSizeArray(types::FixedSizeArrayType {
            element_type, size, ..
        }) => {
            let (element, struct_id) = json_type(*element_type, semantic, library);
            (format!("{element}[{size}]"), struct_id)
        }
        types::Type::ArraySlice(types::ArraySliceType { array_type_id }) => {
            json_type(*array_type_id, semantic, library)
        }
        types::Type::UserDefinedValue(types::UserDefinedValueType { definition_id }) => {
            let Some(binder::Definition::UserDefinedValueType(definition)) =
                semantic.binder().find_definition_by_id(*definition_id)
            else {
                unreachable!("a user-defined value type resolves to its definition");
            };
            let target_type_id = definition
                .target_type_id
                .expect("a user-defined value type in the ABI has a resolved underlying type");
            json_type(target_type_id, semantic, library)
        }
        types::Type::Struct(types::StructType { definition_id, .. }) => {
            ("tuple".to_string(), Some(*definition_id))
        }
        types::Type::Contract(_) | types::Type::Enum(_) | types::Type::Interface(_) if library => {
            (semantic.type_internal_name(type_id), None)
        }
        _ => (
            type_as_abi_type(semantic, type_id)
                .expect("a scalar in the ABI has an ABI type")
                .to_string(),
            None,
        ),
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
