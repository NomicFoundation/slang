use std::rc::Rc;

use metaslang_cst::nodes::NodeId;

use crate::backend::l1_typed_cst::{
    self, ArrayTypeName, ElementaryType, FunctionType, FunctionTypeAttribute, IdentifierPath,
    MappingKeyType, MappingType, Parameter, TypeName,
};
use crate::backend::types::{
    ContractType, DataLocation, EnumType, FunctionTypeKind, InterfaceType, StateVariable,
    StructField, StructType, Type, TypeDefinition, TypeError, TypeId, TypeRegistry,
    UserDefinedValueType,
};
use crate::bindings::{BindingGraph, BindingLocation};
use crate::cst::NonterminalKind;

#[derive(Debug)]
pub struct Error {
    pub node_id: NodeId,
    pub type_error: TypeError,
}

pub struct Pass {
    binding_graph: Rc<BindingGraph>,
    pub types: TypeRegistry,
    pub errors: Vec<Error>,
}

impl Pass {
    pub fn new(binding_graph: Rc<BindingGraph>) -> Self {
        Self {
            types: TypeRegistry::default(),
            binding_graph,
            errors: Vec::new(),
        }
    }

    fn find_or_register_type_name(
        &mut self,
        type_name: &TypeName,
        location: Option<DataLocation>,
    ) -> Result<TypeId, TypeError> {
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
                        // Mapping types can only have data location storage
                        return Err(TypeError::InvalidDataLocation);
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
    ) -> Result<TypeId, TypeError> {
        let element_type_id =
            self.find_or_register_type_name(&array_type_name.operand, location)?;
        let Some(location) = location else {
            // Cannot register array type without a data location
            return Err(TypeError::MissingDataLocation);
        };
        Ok(self.types.register_type(Type::Array {
            element_type: element_type_id,
            location,
        }))
    }

    fn find_or_register_elementary_type(
        &mut self,
        elementary_type: &ElementaryType,
        location: Option<DataLocation>,
    ) -> Result<TypeId, TypeError> {
        let built_in_type = if let Some(location) = location {
            elementary_type.to_type_with_location(location)?
        } else {
            elementary_type.try_to_type()?
        };
        Ok(self.types.register_type(built_in_type))
    }

    fn find_or_register_function_type(
        &mut self,
        function_type: &FunctionType,
    ) -> Result<TypeId, TypeError> {
        let parameters_iter = function_type
            .parameters
            .parameters
            .iter()
            .map(|parameter| self.find_or_register_parameter(parameter));
        let mut parameter_types = Vec::new();
        for parameter in parameters_iter {
            parameter_types.push(parameter?);
        }

        let mut return_types = Vec::new();
        if let Some(return_params_iter) = function_type.returns.as_ref().map(|returns| {
            returns
                .variables
                .parameters
                .iter()
                .map(|return_parameter| self.find_or_register_parameter(return_parameter))
        }) {
            for parameter in return_params_iter {
                return_types.push(parameter?);
            }
        }

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

        Ok(self.types.register_type(Type::Function {
            parameter_types,
            return_types,
            external,
            kind,
        }))
    }

    fn find_or_register_parameter(&mut self, parameter: &Parameter) -> Result<TypeId, TypeError> {
        let parameter_location = parameter
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        self.find_or_register_type_name(&parameter.type_name, parameter_location)
    }

    fn find_or_register_mapping_type(
        &mut self,
        mapping_type: &MappingType,
    ) -> Result<TypeId, TypeError> {
        let key_name = mapping_type
            .key_type
            .name
            .as_ref()
            .map(|key_name| key_name.text.clone());
        let key_type_id = match &mapping_type.key_type.key_type {
            MappingKeyType::ElementaryType(elementary_type) => {
                self.find_or_register_elementary_type(elementary_type, Some(DataLocation::Storage))?
            }
            MappingKeyType::IdentifierPath(identifier_path) => {
                self.find_or_register_identifier_path(identifier_path, Some(DataLocation::Storage))?
            }
        };
        let value_name = mapping_type
            .value_type
            .name
            .as_ref()
            .map(|value_name| value_name.text.clone());
        let value_type_id = self.find_or_register_type_name(
            &mapping_type.value_type.type_name,
            Some(DataLocation::Storage),
        )?;

        Ok(self.types.register_type(Type::Mapping {
            key_name,
            key_type_id,
            value_name,
            value_type_id,
        }))
    }

    fn find_or_register_identifier_path(
        &mut self,
        identifier_path: &IdentifierPath,
        location: Option<DataLocation>,
    ) -> Result<TypeId, TypeError> {
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
            return Err(TypeError::UnresolvedTypeDefinition);
        }
        if definitions.len() > 1 {
            return Err(TypeError::AmbiguousTypeDefinition);
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
                Ok(self.types.register_type(Type::Contract { node_id }))
            }
            NonterminalKind::InterfaceDefinition => {
                Ok(self.types.register_type(Type::Interface { node_id }))
            }
            NonterminalKind::EnumDefinition => Ok(self.types.register_type(Type::Enum { node_id })),
            NonterminalKind::StructDefinition => {
                let Some(location) = location else {
                    // Cannot register type of struct without a data location
                    return Err(TypeError::MissingDataLocation);
                };
                Ok(self.types.register_type(Type::Struct { node_id, location }))
            }
            NonterminalKind::UserDefinedValueTypeDefinition => Ok(self
                .types
                .register_type(Type::UserDefinedValueType { node_id })),
            NonterminalKind::FunctionDefinition => {
                // Cannot register type of a FunctionDefinition
                Err(TypeError::InvalidUserType)
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
}

impl l1_typed_cst::visitor::Visitor for Pass {
    fn leave_contract_definition(&mut self, target: &l1_typed_cst::ContractDefinition) {
        let state_variables = target
            .members
            .iter()
            .filter_map(|member| {
                if let l1_typed_cst::ContractMember::StateVariableDefinition(state_variable) =
                    member
                {
                    match self.find_or_register_type_name(
                        &state_variable.type_name,
                        Some(DataLocation::Storage),
                    ) {
                        Ok(type_id) => Some(StateVariable {
                            node_id: state_variable.node_id,
                            name: state_variable.name.unparse(),
                            type_id,
                        }),
                        Err(type_error) => {
                            self.errors.push(Error {
                                node_id: state_variable.node_id,
                                type_error,
                            });
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Contract(ContractType {
                node_id: target.node_id,
                name: target.name.unparse(),
                state_variables,
            }));
    }

    fn leave_interface_definition(&mut self, target: &l1_typed_cst::InterfaceDefinition) {
        self.types
            .register_definition(TypeDefinition::Interface(InterfaceType {
                node_id: target.node_id,
                name: target.name.unparse(),
            }));
    }

    fn leave_struct_definition(&mut self, target: &l1_typed_cst::StructDefinition) {
        let fields = target
            .members
            .iter()
            .filter_map(|member| {
                match self
                    .find_or_register_type_name(&member.type_name, Some(DataLocation::Inherited))
                {
                    Ok(type_id) => Some(StructField {
                        node_id: member.node_id,
                        name: member.name.unparse(),
                        type_id,
                    }),
                    Err(type_error) => {
                        self.errors.push(Error {
                            node_id: member.node_id,
                            type_error,
                        });
                        None
                    }
                }
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Struct(StructType {
                node_id: target.node_id,
                name: target.name.unparse(),
                fields,
            }));
    }

    fn leave_enum_definition(&mut self, target: &l1_typed_cst::EnumDefinition) {
        let members = target
            .members
            .iter()
            .map(|member| crate::backend::types::EnumMember {
                node_id: member.id(),
                name: member.text.clone(),
            })
            .collect();
        self.types
            .register_definition(TypeDefinition::Enum(EnumType {
                node_id: target.node_id,
                name: target.name.unparse(),
                members,
            }));
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        target: &l1_typed_cst::UserDefinedValueTypeDefinition,
    ) {
        match target.value_type.try_to_type() {
            Ok(value_type) => {
                let value_type_id = self.types.register_type(value_type);
                self.types
                    .register_definition(TypeDefinition::UserDefinedValueType(
                        UserDefinedValueType {
                            node_id: target.node_id,
                            name: target.name.unparse(),
                            type_id: value_type_id,
                        },
                    ));
            }
            Err(type_error) => {
                self.errors.push(Error {
                    node_id: target.node_id,
                    type_error,
                });
            }
        }
    }

    fn leave_parameter(&mut self, target: &l1_typed_cst::Parameter) {
        if let Err(type_error) = self.find_or_register_parameter(target) {
            self.errors.push(Error {
                node_id: target.node_id,
                type_error,
            });
        }
    }

    fn leave_variable_declaration_statement(
        &mut self,
        target: &l1_typed_cst::VariableDeclarationStatement,
    ) {
        let location = target
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        let l1_typed_cst::VariableDeclarationType::TypeName(type_name) = &target.variable_type;
        if let Err(type_error) = self.find_or_register_type_name(type_name, location) {
            self.errors.push(Error {
                node_id: target.node_id,
                type_error,
            });
        }
    }

    fn leave_typed_tuple_member(&mut self, target: &l1_typed_cst::TypedTupleMember) {
        let location = target
            .storage_location
            .as_ref()
            .map(|loc| loc.to_data_location());
        if let Err(type_error) = self.find_or_register_type_name(&target.type_name, location) {
            self.errors.push(Error {
                node_id: target.node_id,
                type_error,
            });
        }
    }
}
