use std::collections::HashMap;
use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use super::p1_flatten_contracts::Output as Input;
use crate::backend::l2_flat_contracts::visitor::{accept_source_unit, Visitor};
use crate::backend::l2_flat_contracts::{
    self as input_ir, ArrayTypeName, ElementaryType, FunctionType, FunctionTypeAttribute,
    IdentifierPath, MappingKeyType, MappingType, Parameter, SourceUnit, StorageLocation, TypeName,
};
use crate::backend::types::{
    ContractTypeDefinition, DataLocation, EnumTypeDefinition, FunctionTypeKind,
    InterfaceTypeDefinition, StateVariable, StructField, StructTypeDefinition, Type,
    TypeDefinition, TypeId, TypeRegistry, UserDefinedValueTypeDefinition,
};
use crate::bindings::{BindingGraph, BindingLocation};
use crate::cst::NonterminalKind;

pub struct Output {
    pub files: HashMap<String, SourceUnit>,
    pub binding_graph: Rc<BindingGraph>,
    pub types: TypeRegistry,
    pub definition_types: HashMap<NodeId, TypeId>,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let binding_graph = input.binding_graph;
    let mut pass = Pass::new(Rc::clone(&binding_graph));
    for source_unit in files.values() {
        accept_source_unit(source_unit, &mut pass);
    }
    pass.types.validate();
    let types = pass.types;
    let definition_types = pass.definition_types;

    Output {
        files,
        binding_graph,
        types,
        definition_types,
    }
}

struct Pass {
    binding_graph: Rc<BindingGraph>,

    /// Contains all types found in the compilation unit
    types: TypeRegistry,
    /// Types (as registered in the `TypeRegistry` above) for (state) variables, parameters and fields
    definition_types: HashMap<NodeId, TypeId>,
}

impl Pass {
    fn new(binding_graph: Rc<BindingGraph>) -> Self {
        let mut types = TypeRegistry::default();

        // Register some always required built-in types
        types.register_type(Type::Boolean);
        types.register_type(Type::Integer {
            signed: false,
            bits: 256,
        });
        types.register_type(Type::Rational);
        types.register_type(Type::Void);

        Self {
            binding_graph,
            types,
            definition_types: HashMap::new(),
        }
    }

    fn find_or_register_type_name(
        &mut self,
        type_name: &TypeName,
        location: Option<DataLocation>,
    ) -> TypeId {
        match type_name {
            TypeName::ArrayTypeName(array_type_name) => {
                self.find_or_register_array_type_name(array_type_name, location)
            }
            TypeName::ElementaryType(elementary_type) => {
                self.find_or_register_elementary_type(elementary_type, location)
            }
            TypeName::FunctionType(function_type) => {
                self.find_or_register_function_type(function_type)
            }
            TypeName::IdentifierPath(identifier_path) => {
                self.find_or_register_identifier_path(identifier_path, location)
            }
            TypeName::MappingType(mapping_type) => {
                match location {
                    // inherited location is ok for mappings in fields of structs
                    Some(DataLocation::Storage | DataLocation::Inherited) => {}
                    _ => {
                        unimplemented!("mapping types can only have storage data location");
                    }
                }
                self.find_or_register_mapping_type(mapping_type)
            }
        }
    }

    fn find_or_register_array_type_name(
        &mut self,
        array_type_name: &ArrayTypeName,
        location: Option<DataLocation>,
    ) -> TypeId {
        let element_type_id = self.find_or_register_type_name(&array_type_name.operand, location);
        let Some(location) = location else {
            unimplemented!("cannot register array type without a data location specifier");
        };
        self.types.register_type(Type::Array {
            element_type: element_type_id,
            location,
        })
    }

    fn find_or_register_elementary_type(
        &mut self,
        elementary_type: &ElementaryType,
        location: Option<DataLocation>,
    ) -> TypeId {
        let built_in_type = elementary_type.to_type(location);
        self.types.register_type(built_in_type)
    }

    fn find_or_register_function_type(&mut self, function_type: &FunctionType) -> TypeId {
        let parameter_types = function_type
            .parameters
            .parameters
            .iter()
            .map(|parameter| self.find_or_register_parameter(parameter))
            .collect::<Vec<_>>();

        let return_type = function_type
            .returns
            .as_ref()
            .map_or(Type::Void, |returns| {
                let types = returns
                    .variables
                    .parameters
                    .iter()
                    .map(|return_parameter| self.find_or_register_parameter(return_parameter))
                    .collect::<Vec<_>>();
                Type::Tuple { types }
            });
        let return_type = self.types.register_type(return_type);

        // TODO: validate that attributes are non-conflicting
        let mut kind = FunctionTypeKind::Pure;
        let mut external = false;
        for attribute in &function_type.attributes {
            match attribute {
                FunctionTypeAttribute::ExternalKeyword => external = true,
                FunctionTypeAttribute::PureKeyword => kind = FunctionTypeKind::Pure,
                FunctionTypeAttribute::ViewKeyword => kind = FunctionTypeKind::View,
                FunctionTypeAttribute::PayableKeyword => kind = FunctionTypeKind::Payable,
                _ => {}
            }
        }

        self.types.register_type(Type::Function {
            parameter_types,
            return_type,
            external,
            kind,
        })
    }

    fn find_or_register_parameter(&mut self, parameter: &Parameter) -> TypeId {
        let parameter_location = parameter
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        self.find_or_register_type_name(&parameter.type_name, parameter_location)
    }

    fn find_or_register_mapping_type(&mut self, mapping_type: &MappingType) -> TypeId {
        let key_name = mapping_type
            .key_type
            .name
            .as_ref()
            .map(|key_name| key_name.text.clone());
        let key_type_id =
            match &mapping_type.key_type.key_type {
                MappingKeyType::ElementaryType(elementary_type) => self
                    .find_or_register_elementary_type(elementary_type, Some(DataLocation::Storage)),
                MappingKeyType::IdentifierPath(identifier_path) => self
                    .find_or_register_identifier_path(identifier_path, Some(DataLocation::Storage)),
            };
        let value_name = mapping_type
            .value_type
            .name
            .as_ref()
            .map(|value_name| value_name.text.clone());
        let value_type_id = self.find_or_register_type_name(
            &mapping_type.value_type.type_name,
            Some(DataLocation::Storage),
        );

        self.types.register_type(Type::Mapping {
            key_name,
            key_type_id,
            value_name,
            value_type_id,
        })
    }

    fn find_or_register_identifier_path(
        &mut self,
        identifier_path: &IdentifierPath,
        location: Option<DataLocation>,
    ) -> TypeId {
        let type_identifier = identifier_path.last().unwrap();
        let type_reference = self
            .binding_graph
            .reference_by_node_id(type_identifier.id())
            .unwrap_or_else(|| {
                // The binding graph should contain a Reference for the
                // identifier. This indicates a bug in the binding rules.
                unreachable!("There is no reference in `Identifier` at path {identifier_path:?}")
            });
        let definitions = type_reference.definitions();
        if definitions.is_empty() {
            unimplemented!("unresolved type definition");
        }
        if definitions.len() > 1 {
            unimplemented!("ambiguous type definition");
        }
        let target_node = match definitions[0].definiens_location() {
            BindingLocation::BuiltIn(_) => {
                // Built-in types should not be referenceable via identifiers
                unreachable!("Unable to register type for a built-in type");
            }
            metaslang_bindings::BindingLocation::UserFile(user_file_location) => {
                user_file_location.cursor().node()
            }
        };
        let Some(target_node) = target_node.as_nonterminal() else {
            // By construction, the definiens should a be a non-terminal. This
            // indicates a bug in binding rules.
            unreachable!("Definiens node {target_node:?} is not a non-terminal");
        };
        let node_id = target_node.id();
        match target_node.kind {
            NonterminalKind::ContractDefinition => {
                self.types.register_type(Type::Contract { node_id })
            }
            NonterminalKind::InterfaceDefinition => {
                self.types.register_type(Type::Interface { node_id })
            }
            NonterminalKind::EnumDefinition => self.types.register_type(Type::Enum { node_id }),
            NonterminalKind::StructDefinition => {
                let Some(location) = location else {
                    unimplemented!(
                        "cannot register type of struct without a data location specifier"
                    );
                };
                self.types.register_type(Type::Struct { node_id, location })
            }
            NonterminalKind::UserDefinedValueTypeDefinition => self
                .types
                .register_type(Type::UserDefinedValueType { node_id }),
            NonterminalKind::FunctionDefinition => {
                unimplemented!("cannot register type of a FunctionDefinition");
            }
            _ => {
                // The definiens node is not of a well-known kind. This
                // indicates a bug in the binding rules.
                unreachable!(
                    "Don't know how to register type for a {kind} node",
                    kind = target_node.kind
                )
            }
        }
    }

    fn register_definition_type(&mut self, identifier_node_id: NodeId, type_id: TypeId) {
        if let Some(definition) = self.binding_graph.definition_by_node_id(identifier_node_id) {
            self.definition_types.insert(definition.id(), type_id);
        }
    }
}

impl Visitor for Pass {
    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        let state_variables = node
            .members
            .iter()
            .filter_map(|member| {
                if let input_ir::ContractMember::StateVariableDefinition(state_variable) = member {
                    let type_id = self.find_or_register_type_name(
                        &state_variable.type_name,
                        Some(DataLocation::Storage),
                    );
                    self.register_definition_type(state_variable.name.id(), type_id);
                    Some(StateVariable {
                        node_id: state_variable.node_id,
                        name: state_variable.name.unparse(),
                        type_id,
                    })
                } else {
                    None
                }
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Contract(ContractTypeDefinition {
                node_id: node.node_id,
                name: node.name.unparse(),
                state_variables,
            }));
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.types
            .register_definition(TypeDefinition::Interface(InterfaceTypeDefinition {
                node_id: node.node_id,
                name: node.name.unparse(),
            }));
    }

    fn enter_struct_definition(&mut self, node: &input_ir::StructDefinition) -> bool {
        let fields = node
            .members
            .iter()
            .map(|member| {
                let type_id = self
                    .find_or_register_type_name(&member.type_name, Some(DataLocation::Inherited));
                self.register_definition_type(member.name.id(), type_id);
                StructField {
                    node_id: member.node_id,
                    name: member.name.unparse(),
                    type_id,
                }
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Struct(StructTypeDefinition {
                node_id: node.node_id,
                name: node.name.unparse(),
                fields,
            }));
        false
    }

    fn enter_enum_definition(&mut self, node: &input_ir::EnumDefinition) -> bool {
        let members = node
            .members
            .iter()
            .map(|member| crate::backend::types::EnumMember {
                node_id: member.id(),
                name: member.text.clone(),
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Enum(EnumTypeDefinition {
                node_id: node.node_id,
                name: node.name.unparse(),
                members,
            }));

        // Add definition types for all enum members
        let enum_type_id = self.types.register_type(Type::Enum {
            node_id: node.node_id,
        });
        for member in &node.members {
            self.definition_types.insert(member.id(), enum_type_id);
        }

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let value_type = node.value_type.to_type(None);
        let value_type_id = self.types.register_type(value_type);
        self.types
            .register_definition(TypeDefinition::UserDefinedValueType(
                UserDefinedValueTypeDefinition {
                    node_id: node.node_id,
                    name: node.name.unparse(),
                    type_id: value_type_id,
                },
            ));
        false
    }

    fn enter_parameter(&mut self, node: &input_ir::Parameter) -> bool {
        let type_id = self.find_or_register_parameter(node);
        if let Some(ref name) = node.name {
            self.register_definition_type(name.id(), type_id);
        }
        false
    }

    fn enter_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) -> bool {
        let location = node
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        let input_ir::VariableDeclarationType::TypeName(type_name) = &node.variable_type;
        let type_id = self.find_or_register_type_name(type_name, location);
        self.register_definition_type(node.name.id(), type_id);
        false
    }

    fn enter_typed_tuple_member(&mut self, node: &input_ir::TypedTupleMember) -> bool {
        let location = node
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        let type_id = self.find_or_register_type_name(&node.type_name, location);
        self.register_definition_type(node.name.id(), type_id);
        false
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        let parameter_types = node
            .parameters
            .parameters
            .iter()
            .map(|parameter| self.find_or_register_parameter(parameter))
            .collect();
        let return_type = node.returns.as_ref().map_or(Type::Void, |returns| {
            let types = returns
                .variables
                .parameters
                .iter()
                .map(|parameter| self.find_or_register_parameter(parameter))
                .collect();
            Type::Tuple { types }
        });
        let return_type = self.types.register_type(return_type);

        // TODO: validate that attributes are non-conflicting
        let mut kind = FunctionTypeKind::Pure;
        let mut external = false;
        for attribute in &node.attributes {
            match attribute {
                input_ir::FunctionAttribute::ExternalKeyword => external = true,
                input_ir::FunctionAttribute::PureKeyword => kind = FunctionTypeKind::Pure,
                input_ir::FunctionAttribute::ViewKeyword => kind = FunctionTypeKind::View,
                input_ir::FunctionAttribute::PayableKeyword => kind = FunctionTypeKind::Payable,
                _ => {}
            }
        }

        let type_id = self.types.register_type(Type::Function {
            parameter_types,
            return_type,
            external,
            kind,
        });
        if let input_ir::FunctionName::Identifier(ref name) = node.name {
            self.register_definition_type(name.id(), type_id);
        }
    }
}

impl ElementaryType {
    fn to_type(&self, location: Option<DataLocation>) -> Type {
        match self {
            Self::AddressType(address) => {
                let payable = address.payable_keyword.is_some();
                Type::Address { payable }
            }
            Self::BoolKeyword => Type::Boolean,
            Self::BytesKeyword(bytes) => {
                if bytes.text == "bytes" {
                    let Some(location) = location else {
                        unimplemented!("bytes type is missing a data location specifier");
                    };
                    Type::Bytes { location }
                } else {
                    let width = bytes.text.strip_prefix("bytes").unwrap().parse().unwrap();
                    Type::ByteArray { width }
                }
            }
            Self::FixedKeyword(fixed) => {
                let parts: Vec<_> = fixed
                    .text
                    .strip_prefix("fixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect();
                let (bits, precision_bits) = if parts.is_empty() {
                    (128, 18)
                } else {
                    (parts[0], parts[1])
                };
                Type::FixedPointNumber {
                    signed: true,
                    bits,
                    precision_bits,
                }
            }
            Self::UfixedKeyword(ufixed) => {
                let parts: Vec<_> = ufixed
                    .text
                    .strip_prefix("ufixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap())
                    .collect();
                let (bits, precision_bits) = if parts.is_empty() {
                    (128, 18)
                } else {
                    (parts[0], parts[1])
                };
                Type::FixedPointNumber {
                    signed: true,
                    bits,
                    precision_bits,
                }
            }
            Self::IntKeyword(int) => {
                let bits = int
                    .text
                    .strip_prefix("int")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Type::Integer { signed: true, bits }
            }
            Self::UintKeyword(uint) => {
                let bits = uint
                    .text
                    .strip_prefix("uint")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Type::Integer {
                    signed: false,
                    bits,
                }
            }
            Self::StringKeyword => {
                let Some(location) = location else {
                    unimplemented!("Missing data location specifier for string type");
                };
                Type::String { location }
            }
        }
    }
}

impl StorageLocation {
    fn to_data_location(&self) -> DataLocation {
        match self {
            Self::CallDataKeyword => DataLocation::Calldata,
            Self::MemoryKeyword => DataLocation::Memory,
            Self::StorageKeyword => DataLocation::Storage,
        }
    }
}
