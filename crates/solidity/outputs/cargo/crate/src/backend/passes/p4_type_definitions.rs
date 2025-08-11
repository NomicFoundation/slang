use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p3_linearise_contracts::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, Reference, Resolution, Scope, ScopeId, Typing, UsingDirective,
};
use crate::backend::built_ins::BuiltIn;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, FunctionTypeKind, Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::{NodeId, TerminalKind};

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(input.binder, compilation_unit.language_version());
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }
    for source_unit in files.values() {
        pass.visit_file_type_getters(source_unit);
    }
    let binder = pass.binder;
    let types = pass.types;

    Output {
        compilation_unit,
        files,
        binder,
        types,
    }
}

fn storage_location_to_data_location(storage_location: &input_ir::StorageLocation) -> DataLocation {
    match storage_location {
        input_ir::StorageLocation::MemoryKeyword => DataLocation::Memory,
        input_ir::StorageLocation::StorageKeyword => DataLocation::Storage,
        input_ir::StorageLocation::CallDataKeyword => DataLocation::Calldata,
    }
}

const VERSION_0_5_0: Version = Version::new(0, 5, 0);

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types: TypeRegistry::default(),
        }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        assert!(self.scope_stack.is_empty());
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(scope_id);
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            current_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_contract_or_file_scope_id(&self) -> ScopeId {
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            if matches!(scope, Scope::Contract(_) | Scope::File(_)) {
                return *scope_id;
            }
        }
        unreachable!("attempt to get the current contract scope without a contract or file scope in the stack");
    }

    // Resolves an IdentifierPath. It starts resolution at the "contract" scope
    // level, or at the file level if there's no contract scope open. It will
    // follow through in contracts/intrefaces/libraries as well as imports and
    // treat them as namespaces.
    // Returns the resolution of the last reference.
    fn resolve_identifier_path(
        &mut self,
        identifier_path: &input_ir::IdentifierPath,
    ) -> Resolution {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let mut scope_id = Some(self.current_contract_or_file_scope_id());
        let mut use_lexical_resolution = true;
        let mut last_resolution: Resolution = Resolution::Unresolved;

        for identifier in identifier_path {
            let symbol = identifier.unparse();
            let resolution = if let Some(scope_id) = scope_id {
                if use_lexical_resolution {
                    self.binder.resolve_in_scope(scope_id, &symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, &symbol)
                }
            } else {
                Resolution::Unresolved
            };
            let definition_id = resolution.as_definition_id();

            let reference = Reference::new(Rc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            // recurse into file scopes pointed by the resolved definition
            // to resolve the next identifier in the path
            scope_id = definition_id
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| match definition {
                    Definition::Import(ImportDefinition {
                        resolved_file_id, ..
                    }) => resolved_file_id.as_ref().and_then(|resolved_file_id| {
                        self.binder.scope_id_for_file_id(resolved_file_id)
                    }),
                    Definition::Contract(ContractDefinition { node_id, .. })
                    | Definition::Interface(InterfaceDefinition { node_id, .. })
                    | Definition::Library(LibraryDefinition { node_id, .. }) => {
                        use_lexical_resolution = false;
                        self.binder.scope_id_for_node_id(*node_id)
                    }
                    _ => None,
                });

            last_resolution = resolution;
        }
        last_resolution
    }

    fn resolve_parameter_types(
        &mut self,
        parameters: &input_ir::Parameters,
    ) -> Option<Vec<TypeId>> {
        let mut parameter_types = Vec::new();
        for parameter in parameters {
            let parameter_type_id = self.binder.node_typing(parameter.node_id).as_type_id()?;
            parameter_types.push(parameter_type_id);
        }
        Some(parameter_types)
    }

    fn resolve_type_name(
        &mut self,
        type_name: &input_ir::TypeName,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match type_name {
            input_ir::TypeName::ArrayTypeName(array_type_name) => {
                data_location.and_then(|data_location| {
                    self.resolve_type_name(&array_type_name.operand, Some(data_location))
                        .map(|element_type| {
                            self.types.register_type(Type::Array {
                                element_type,
                                location: data_location,
                            })
                        })
                })
            }
            input_ir::TypeName::FunctionType(function_type) => {
                // NOTE: Keep in sync with `type_of_function_definition`
                let parameter_types =
                    self.resolve_parameter_types(&function_type.parameters.parameters)?;
                let return_type = if let Some(returns) = &function_type.returns {
                    let return_types =
                        self.resolve_parameter_types(&returns.variables.parameters)?;
                    if return_types.len() == 1 {
                        return_types.first().copied().unwrap()
                    } else {
                        self.types.register_type(Type::Tuple {
                            types: return_types,
                        })
                    }
                } else {
                    self.types.void()
                };
                let mut kind = FunctionTypeKind::Pure;
                let mut external = false;
                for attribute in &function_type.attributes {
                    match attribute {
                        input_ir::FunctionTypeAttribute::ExternalKeyword
                        | input_ir::FunctionTypeAttribute::PublicKeyword => external = true,
                        input_ir::FunctionTypeAttribute::PureKeyword => {
                            kind = FunctionTypeKind::Pure;
                        }
                        input_ir::FunctionTypeAttribute::ViewKeyword => {
                            kind = FunctionTypeKind::View;
                        }
                        input_ir::FunctionTypeAttribute::PayableKeyword => {
                            kind = FunctionTypeKind::Payable;
                        }
                        _ => {}
                    }
                }
                Some(self.types.register_type(Type::Function {
                    definition_id: None,
                    parameter_types,
                    return_type,
                    external,
                    kind,
                }))
            }
            input_ir::TypeName::MappingType(mapping_type) => {
                let data_location = Some(DataLocation::Storage);
                let key_type_id = match &mapping_type.key_type.key_type {
                    input_ir::MappingKeyType::ElementaryType(elementary_type) => {
                        self.type_of_elementary_type(elementary_type, data_location)
                    }
                    input_ir::MappingKeyType::IdentifierPath(identifier_path) => {
                        self.type_of_identifier_path(identifier_path, data_location)
                    }
                }?;
                let value_type_id =
                    self.resolve_type_name(&mapping_type.value_type.type_name, data_location)?;
                Some(self.types.register_type(Type::Mapping {
                    key_type_id,
                    value_type_id,
                }))
            }
            input_ir::TypeName::ElementaryType(elementary_type) => {
                self.type_of_elementary_type(elementary_type, data_location)
            }
            input_ir::TypeName::IdentifierPath(identifier_path) => {
                self.type_of_identifier_path(identifier_path, data_location)
            }
        }
    }

    fn type_of_identifier_path(
        &mut self,
        identifier_path: &input_ir::IdentifierPath,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| reference.resolution.as_definition_id())
            .and_then(|node_id| self.type_of_definition(node_id, data_location))
    }

    fn type_of_definition(
        &mut self,
        definition_id: NodeId,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        if let Some(definition) = self.binder.find_definition_by_id(definition_id) {
            match definition {
                Definition::Contract(_) | Definition::Library(_) => {
                    // TODO: do we need a separate type for libraries?
                    Some(self.types.register_type(Type::Contract { definition_id }))
                }
                Definition::Enum(_) => Some(self.types.register_type(Type::Enum { definition_id })),
                Definition::Interface(_) => {
                    Some(self.types.register_type(Type::Interface { definition_id }))
                }
                Definition::Struct(_) => data_location.map(|data_location| {
                    self.types.register_type(Type::Struct {
                        definition_id,
                        location: data_location,
                    })
                }),
                Definition::TypeParameter(_) => {
                    unreachable!("type parameter names don't have a type")
                }
                Definition::UserDefinedValueType(_) => Some(
                    self.types
                        .register_type(Type::UserDefinedValue { definition_id }),
                ),

                Definition::Constant(_)
                | Definition::EnumMember(_)
                | Definition::Error(_)
                | Definition::Event(_)
                | Definition::Function(_)
                | Definition::Import(_)
                | Definition::ImportedSymbol(_)
                | Definition::Modifier(_)
                | Definition::Parameter(_)
                | Definition::StateVariable(_)
                | Definition::StructMember(_)
                | Definition::Variable(_)
                | Definition::YulFunction(_)
                | Definition::YulLabel(_)
                | Definition::YulParameter(_)
                | Definition::YulVariable(_) => None,
            }
        } else {
            None
        }
    }

    fn type_of_elementary_type(
        &mut self,
        elementary_type: &input_ir::ElementaryType,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match elementary_type {
            input_ir::ElementaryType::AddressType(address_type) => {
                Some(self.types.register_type(Type::Address {
                    payable: address_type.payable_keyword.is_some(),
                }))
            }
            input_ir::ElementaryType::BytesKeyword(bytes_keyword) => {
                Type::from_bytes_keyword(&bytes_keyword.unparse(), data_location)
                    .map(|type_| self.types.register_type(type_))
            }
            input_ir::ElementaryType::IntKeyword(int_keyword) => Some(
                self.types
                    .register_type(Type::from_int_keyword(&int_keyword.unparse())),
            ),
            input_ir::ElementaryType::UintKeyword(uint_keyword) => Some(
                self.types
                    .register_type(Type::from_uint_keyword(&uint_keyword.unparse())),
            ),
            input_ir::ElementaryType::FixedKeyword(fixed_keyword) => Some(
                self.types
                    .register_type(Type::from_fixed_keyword(&fixed_keyword.unparse())),
            ),
            input_ir::ElementaryType::UfixedKeyword(ufixed_keyword) => Some(
                self.types
                    .register_type(Type::from_ufixed_keyword(&ufixed_keyword.unparse())),
            ),
            input_ir::ElementaryType::BoolKeyword => Some(self.types.boolean()),
            input_ir::ElementaryType::ByteKeyword => {
                Some(self.types.register_type(Type::ByteArray { width: 1 }))
            }
            input_ir::ElementaryType::StringKeyword => data_location.map(|data_location| {
                self.types.register_type(Type::String {
                    location: data_location,
                })
            }),
        }
    }

    fn type_of_function_definition(
        &mut self,
        function_definition: &input_ir::FunctionDefinition,
    ) -> Option<TypeId> {
        // NOTE: Keep in sync with function types in `resolve_type_name` above
        let parameter_types =
            self.resolve_parameter_types(&function_definition.parameters.parameters)?;
        let return_type = if let Some(returns) = &function_definition.returns {
            let return_types = self.resolve_parameter_types(&returns.variables.parameters)?;
            if return_types.len() == 1 {
                // TODO: I think this is more correct, but we may decide to
                // always return a tuple instead; this will require changing the
                // typing of function call expressions
                return_types.first().copied().unwrap()
            } else {
                self.types.register_type(Type::Tuple {
                    types: return_types,
                })
            }
        } else {
            self.types.void()
        };
        let mut kind = FunctionTypeKind::Pure;
        let mut external = false;
        for attribute in &function_definition.attributes {
            match attribute {
                input_ir::FunctionAttribute::ExternalKeyword
                | input_ir::FunctionAttribute::PublicKeyword => external = true,
                input_ir::FunctionAttribute::PureKeyword => {
                    kind = FunctionTypeKind::Pure;
                }
                input_ir::FunctionAttribute::ViewKeyword => {
                    kind = FunctionTypeKind::View;
                }
                input_ir::FunctionAttribute::PayableKeyword => {
                    kind = FunctionTypeKind::Payable;
                }
                _ => {}
            }
        }
        Some(self.types.register_type(Type::Function {
            definition_id: Some(function_definition.node_id),
            parameter_types,
            return_type,
            external,
            kind,
        }))
    }

    fn find_library_scope_id_for_identifier_path(
        &self,
        identifier_path: &input_ir::IdentifierPath,
    ) -> Option<ScopeId> {
        let definition_id = identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| reference.resolution.as_definition_id())?;

        let Some(Definition::Library(_)) = self.binder.find_definition_by_id(definition_id) else {
            // the referenced definition is not a library
            return None;
        };
        self.binder.scope_id_for_node_id(definition_id)
    }

    // This is a short second pass to compute and register the types of the
    // getter functions for public state variables. Computing the type of the
    // getter requires all struct fields to be typed already thus why this
    // cannot happen concurrently with the typing of the definitions in the main
    // pass.
    fn visit_file_type_getters(&mut self, source_unit: &input_ir::SourceUnit) {
        for source_unit_member in &source_unit.members {
            let input_ir::SourceUnitMember::ContractDefinition(contract_definition) =
                source_unit_member
            else {
                continue;
            };
            for contract_member in &contract_definition.members {
                let input_ir::ContractMember::StateVariableDefinition(state_var_definition) =
                    contract_member
                else {
                    continue;
                };
                if state_var_definition.attributes.iter().all(|attribute| {
                    !matches!(attribute, input_ir::StateVariableAttribute::PublicKeyword)
                }) {
                    continue;
                }

                let node_id = state_var_definition.node_id;
                let Some(type_id) = self.binder.node_typing(node_id).as_type_id() else {
                    continue;
                };

                let Some(getter_type_id) = self.compute_getter_type(node_id, type_id) else {
                    continue;
                };
                let Definition::StateVariable(definition) = self.binder.get_definition_mut(node_id)
                else {
                    unreachable!("definition is not a state variable");
                };
                definition.getter_type_id = Some(getter_type_id);
            }
        }
    }

    fn compute_getter_type(&mut self, definition_id: NodeId, type_id: TypeId) -> Option<TypeId> {
        let mut return_type = type_id;
        let mut parameter_types = Vec::new();

        loop {
            match self.types.get_type_by_id(return_type) {
                // scalar types
                Type::Address { .. }
                | Type::Boolean
                | Type::ByteArray { .. }
                | Type::Bytes { .. }
                | Type::Contract { .. }
                | Type::Enum { .. }
                | Type::FixedPointNumber { .. }
                | Type::Function { .. }
                | Type::Integer { .. }
                | Type::Interface { .. }
                | Type::String { .. }
                | Type::UserDefinedValue { .. } => break,

                Type::Struct { definition_id, .. } => {
                    // For structs there is a special case: if it has a single
                    // member, the getter will return the value of that field.
                    // This special case is the reason why we cannot type
                    // getters concurrently with the state variables.

                    // To retrieve that field we need to go through the
                    // scope associated to the struct...
                    let scope_id = self
                        .binder
                        .scope_id_for_node_id(*definition_id)
                        .expect("definition in struct type has no associated scope");
                    let Scope::Struct(struct_scope) = self.binder.get_scope_by_id(scope_id) else {
                        unreachable!("definition in struct type has no invalid scope");
                    };
                    if struct_scope.definitions.len() == 1 {
                        let member_definition_id =
                            struct_scope.definitions.values().next().unwrap();
                        if let Some(member_type_id) =
                            self.binder.node_typing(*member_definition_id).as_type_id()
                        {
                            return_type = member_type_id;
                        } else {
                            return None;
                        }
                    }
                    break;
                }

                // non-scalar types
                Type::Array { element_type, .. } => {
                    return_type = *element_type;
                    parameter_types.push(self.types.uint256());
                }
                Type::Mapping {
                    key_type_id,
                    value_type_id,
                } => {
                    return_type = *value_type_id;
                    parameter_types.push(*key_type_id);
                }

                // invalid types
                Type::Rational | Type::Tuple { .. } | Type::Void => {
                    unreachable!("cannot compute the getter type for {type_id:?}")
                }
            }
        }

        let getter_type = Type::Function {
            definition_id: Some(definition_id),
            parameter_types,
            return_type,
            external: true,
            kind: FunctionTypeKind::View,
        };
        Some(self.types.register_type(getter_type))
    }

    fn visit_parameters(
        &mut self,
        parameters: &input_ir::Parameters,
        default_location: Option<DataLocation>,
    ) {
        for parameter in parameters {
            let type_id = self.resolve_type_name(
                &parameter.type_name,
                parameter
                    .storage_location
                    .as_ref()
                    .map(storage_location_to_data_location)
                    .or(default_location),
            );
            self.binder.set_node_type(parameter.node_id, type_id);
        }
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_path_import(&mut self, node: &input_ir::PathImport) {
        if node.alias.is_some() {
            self.binder.mark_user_meta_type_node(node.node_id);
        }
    }

    fn leave_named_import(&mut self, node: &input_ir::NamedImport) {
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn enter_import_deconstruction(&mut self, node: &input_ir::ImportDeconstruction) -> bool {
        for symbol in &node.symbols {
            let target_symbol = if let Some(alias) = &symbol.alias {
                &alias.identifier
            } else {
                &symbol.name
            };
            let resolution = self.binder.resolve_in_scope(
                self.current_contract_or_file_scope_id(),
                &target_symbol.unparse(),
            );
            let reference = Reference::new(Rc::clone(&symbol.name), resolution);
            self.binder.insert_reference(reference);
        }

        false
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        let default_location = if self.language_version < VERSION_0_5_0 {
            if node.attributes.iter().any(|attribute| {
                matches!(
                    attribute,
                    input_ir::FunctionAttribute::ExternalKeyword
                        | input_ir::FunctionAttribute::PublicKeyword
                )
            }) {
                Some(DataLocation::Calldata)
            } else {
                Some(DataLocation::Memory)
            }
        } else {
            None
        };

        // type parameters and return variables
        self.visit_parameters(&node.parameters.parameters, default_location);
        if let Some(returns) = &node.returns {
            self.visit_parameters(&returns.variables.parameters, default_location);
        }

        // now we can compute the function type
        let type_id = self.type_of_function_definition(node);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_function_type(&mut self, node: &input_ir::FunctionType) {
        self.visit_parameters(&node.parameters.parameters, None);
        if let Some(returns) = &node.returns {
            self.visit_parameters(&returns.variables.parameters, None);
        }
    }

    fn leave_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) {
        let default_location = if self.language_version < VERSION_0_5_0 {
            Some(DataLocation::Calldata)
        } else {
            None
        };
        self.visit_parameters(&node.parameters.parameters, default_location);
    }

    fn leave_unnamed_function_definition(&mut self, node: &input_ir::UnnamedFunctionDefinition) {
        let default_location = if self.language_version < VERSION_0_5_0 {
            Some(DataLocation::Calldata)
        } else {
            None
        };
        self.visit_parameters(&node.parameters.parameters, default_location);
    }

    fn leave_fallback_function_definition(&mut self, node: &input_ir::FallbackFunctionDefinition) {
        self.visit_parameters(&node.parameters.parameters, None);
        if let Some(returns) = &node.returns {
            self.visit_parameters(&returns.variables.parameters, None);
        }
    }

    fn leave_receive_function_definition(&mut self, node: &input_ir::ReceiveFunctionDefinition) {
        self.visit_parameters(&node.parameters.parameters, None);
    }

    fn leave_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) {
        if let Some(parameters) = &node.parameters {
            let default_location = if self.language_version < VERSION_0_5_0 {
                Some(DataLocation::Memory)
            } else {
                None
            };
            self.visit_parameters(&parameters.parameters, default_location);
        }
    }

    fn leave_try_statement(&mut self, node: &input_ir::TryStatement) {
        if let Some(returns) = &node.returns {
            self.visit_parameters(&returns.variables.parameters, None);
        }
    }

    fn leave_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) {
        self.visit_parameters(&node.parameters.parameters, None);
    }

    fn enter_type_name(&mut self, node: &input_ir::TypeName) -> bool {
        if let input_ir::TypeName::IdentifierPath(identifier_path) = node {
            self.resolve_identifier_path(identifier_path);
            false
        } else {
            true
        }
    }

    fn enter_mapping_key_type(&mut self, node: &input_ir::MappingKeyType) -> bool {
        if let input_ir::MappingKeyType::IdentifierPath(identifier_path) = node {
            self.resolve_identifier_path(identifier_path);
            false
        } else {
            true
        }
    }

    fn enter_override_paths(&mut self, items: &input_ir::OverridePaths) -> bool {
        for identifier_path in items {
            self.resolve_identifier_path(identifier_path);
        }
        false
    }

    fn leave_event_definition(&mut self, node: &input_ir::EventDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_event_parameter(&mut self, node: &input_ir::EventParameter) {
        // TODO: the data location is not strictly correct, but strings, bytes
        // and structs are allowed as event parameters and they won't type if we
        // pass None here
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_error_definition(&mut self, node: &input_ir::ErrorDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_error_parameter(&mut self, node: &input_ir::ErrorParameter) {
        // TODO: the data location is not strictly correct, but strings, bytes
        // and structs are allowed as error parameters and they won't type if we
        // pass None here
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_state_variable_definition(&mut self, node: &input_ir::StateVariableDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Storage));
        self.binder.set_node_type(node.node_id, type_id);
        // if required, we will compute the type of the getter after all
        // definitions have been typed
    }

    fn leave_constant_definition(&mut self, node: &input_ir::ConstantDefinition) {
        let type_id = self.resolve_type_name(&node.type_name, None);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) {
        let type_id =
            if let input_ir::VariableDeclarationType::TypeName(type_name) = &node.variable_type {
                self.resolve_type_name(
                    type_name,
                    node.storage_location
                        .as_ref()
                        .map(storage_location_to_data_location)
                        .or_else(|| {
                            if self.language_version < VERSION_0_5_0 {
                                // default data location is storage for variables
                                Some(DataLocation::Storage)
                            } else {
                                None
                            }
                        }),
                )
            } else {
                // this is for `var` variables (in Solidity < 0.5.0)
                // we cannot resolve the type at this point
                None
            };
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_typed_tuple_member(&mut self, node: &input_ir::TypedTupleMember) {
        let type_id = self.resolve_type_name(
            &node.type_name,
            node.storage_location
                .as_ref()
                .map(storage_location_to_data_location)
                .or_else(|| {
                    if self.language_version < VERSION_0_5_0 {
                        // default data location is storage for variables
                        Some(DataLocation::Storage)
                    } else {
                        None
                    }
                }),
        );
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) {
        if node.var_keyword.is_none() {
            return;
        }
        // this handles adding type information to the `var` declarations in Solidity < 0.5.0
        for element in &node.elements {
            let Some(member) = &element.member else {
                continue;
            };
            if let input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) = member {
                // TODO: the variables cannot be typed at this point, but we
                // should be able to infer their type in p4 after computing the
                // value type
                self.binder
                    .set_node_type(untyped_tuple_member.node_id, None);
            }
        }
    }

    fn leave_struct_definition(&mut self, node: &input_ir::StructDefinition) {
        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_struct_member(&mut self, node: &input_ir::StructMember) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Inherited));
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_enum_definition(&mut self, node: &input_ir::EnumDefinition) {
        let type_id = self.types.register_type(Type::Enum {
            definition_id: node.node_id,
        });
        for member in &node.members {
            self.binder.set_node_type(member.id(), Some(type_id));
        }

        self.binder.mark_user_meta_type_node(node.node_id);
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) {
        self.binder.mark_user_meta_type_node(node.node_id);

        let target_type_id = self.type_of_elementary_type(&node.value_type, None);
        let Definition::UserDefinedValueType(udvt) = self.binder.get_definition_mut(node.node_id)
        else {
            unreachable!("definition in UDVT node is not a UDVT");
        };
        udvt.target_type_id = target_type_id;
    }

    fn enter_using_directive(&mut self, node: &input_ir::UsingDirective) -> bool {
        match &node.clause {
            input_ir::UsingClause::IdentifierPath(identifier_path) => {
                self.resolve_identifier_path(identifier_path);
            }
            input_ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
                for symbol in &using_deconstruction.symbols {
                    self.resolve_identifier_path(&symbol.name);
                }
            }
        }
        // target's TypeName is resolved when entering it
        true
    }

    fn leave_using_directive(&mut self, node: &input_ir::UsingDirective) {
        let directive = match &node.target {
            input_ir::UsingTarget::TypeName(type_name) => {
                let Some(type_id) =
                    // TODO: not sure we should be using DataLocation::Inherited
                    // here, but if we don't provide one we can't register
                    // reference types and hence we cannot get a type_id
                    self.resolve_type_name(type_name, Some(DataLocation::Inherited))
                else {
                    return;
                };
                match &node.clause {
                    input_ir::UsingClause::IdentifierPath(identifier_path) => {
                        let Some(scope_id) =
                            self.find_library_scope_id_for_identifier_path(identifier_path)
                        else {
                            return;
                        };
                        UsingDirective::new_single_type(scope_id, type_id)
                    }
                    input_ir::UsingClause::UsingDeconstruction(using_deconstruction) => {
                        let mut symbols = HashMap::new();
                        let mut operators = HashMap::new();

                        for symbol in &using_deconstruction.symbols {
                            let symbol_name = symbol.name.last().unwrap();
                            let definition_ids = self
                                .binder
                                .find_reference_by_identifier_node_id(symbol_name.id())
                                .map_or(Vec::new(), |reference| {
                                    reference.resolution.get_definition_ids()
                                });
                            // TODO(validation): *all* definitions should point to functions

                            symbols.insert(symbol_name.unparse(), definition_ids);

                            if let Some(alias) = &symbol.alias {
                                let terminal = match alias.operator {
                                    input_ir::UsingOperator::Ampersand => TerminalKind::Ampersand,
                                    input_ir::UsingOperator::Asterisk => TerminalKind::Asterisk,
                                    input_ir::UsingOperator::BangEqual => TerminalKind::BangEqual,
                                    input_ir::UsingOperator::Bar => TerminalKind::Bar,
                                    input_ir::UsingOperator::Caret => TerminalKind::Caret,
                                    input_ir::UsingOperator::EqualEqual => TerminalKind::EqualEqual,
                                    input_ir::UsingOperator::GreaterThan => {
                                        TerminalKind::GreaterThan
                                    }
                                    input_ir::UsingOperator::GreaterThanEqual => {
                                        TerminalKind::GreaterThanEqual
                                    }
                                    input_ir::UsingOperator::LessThan => TerminalKind::LessThan,
                                    input_ir::UsingOperator::LessThanEqual => {
                                        TerminalKind::LessThanEqual
                                    }
                                    input_ir::UsingOperator::Minus => TerminalKind::Minus,
                                    input_ir::UsingOperator::Percent => TerminalKind::Percent,
                                    input_ir::UsingOperator::Plus => TerminalKind::Plus,
                                    input_ir::UsingOperator::Slash => TerminalKind::Slash,
                                    input_ir::UsingOperator::Tilde => TerminalKind::Tilde,
                                };
                                operators.insert(terminal, symbol_name.unparse());
                            }
                        }

                        let scope = Scope::new_using(node.node_id, symbols);
                        let scope_id = self.binder.insert_scope(scope);

                        if operators.is_empty() {
                            UsingDirective::new_single_type(scope_id, type_id)
                        } else {
                            UsingDirective::new_single_type_with_operators(
                                scope_id, type_id, operators,
                            )
                        }
                    }
                }
            }
            input_ir::UsingTarget::Asterisk => {
                let input_ir::UsingClause::IdentifierPath(identifier_path) = &node.clause else {
                    // only libraries can be attached to all types
                    return;
                };
                let Some(scope_id) =
                    self.find_library_scope_id_for_identifier_path(identifier_path)
                else {
                    // the identifier path does not point to a valid library
                    return;
                };
                UsingDirective::new_all(scope_id)
            }
        };

        if node.global_keyword.is_some() {
            self.binder.insert_global_using_directive(directive);
        } else {
            let current_scope_id = self.current_contract_or_file_scope_id();
            self.binder
                .insert_using_directive_in_scope(directive, current_scope_id);
        }
    }

    fn enter_revert_statement(&mut self, node: &input_ir::RevertStatement) -> bool {
        if let Some(identifier_path) = &node.error {
            self.resolve_identifier_path(identifier_path);
        }
        true
    }

    fn leave_revert_statement(&mut self, node: &input_ir::RevertStatement) {
        if let Some(identifier_path) = &node.error {
            let type_id = self.type_of_identifier_path(identifier_path, None);
            self.binder.set_node_type(node.node_id, type_id);
        }
    }

    fn enter_emit_statement(&mut self, node: &input_ir::EmitStatement) -> bool {
        self.resolve_identifier_path(&node.event);
        true
    }

    fn leave_emit_statement(&mut self, node: &input_ir::EmitStatement) {
        let type_id = self.type_of_identifier_path(&node.event, None);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn enter_catch_clause_error(&mut self, node: &input_ir::CatchClauseError) -> bool {
        if let Some(name) = &node.name {
            let resolution = match name.unparse().as_str() {
                "Error" | "Panic" => Resolution::BuiltIn(BuiltIn::ErrorOrPanic),
                _ => Resolution::Unresolved,
            };
            let reference = Reference::new(Rc::clone(name), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_new_expression(&mut self, node: &input_ir::NewExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, Typing::New);
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_type_expression(&mut self, node: &input_ir::TypeExpression) {
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        let typing = type_id.map_or(Typing::Unresolved, |type_id| {
            Typing::BuiltIn(BuiltIn::Type(type_id))
        });
        self.binder.set_node_typing(node.node_id, typing);
    }
}
