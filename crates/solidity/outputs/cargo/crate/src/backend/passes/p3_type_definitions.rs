use std::collections::HashMap;
use std::rc::Rc;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, Reference, Resolution, Scope, ScopeId, UsingDirective,
};
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
    let mut pass = Pass::new(input.binder);
    for source_unit in files.values() {
        pass.visit_file(source_unit);
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

struct Pass {
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder) -> Self {
        Self {
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
    fn resolve_identifier_path(&mut self, identifier_path: &input_ir::IdentifierPath) {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let mut scope_id = Some(self.current_contract_or_file_scope_id());
        let mut use_lexical_resolution = true;

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

            let reference = Reference::new(Rc::clone(identifier), resolution);
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
        }
    }

    fn resolve_inheritance_types(&mut self, types: &input_ir::InheritanceTypes) {
        for inheritance_type in types {
            self.resolve_identifier_path(&inheritance_type.type_name);
        }
        // TODO: return the resolved types (ie. the definition for the last
        // identifier in each path)?
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
                        input_ir::FunctionTypeAttribute::ExternalKeyword => external = true,
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
                Definition::Contract(_) => {
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
                | Definition::Library(_)
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
                let width = bytes_keyword
                    .unparse()
                    .strip_prefix("bytes")
                    .unwrap()
                    .parse::<u32>();
                if let Ok(width) = width {
                    Some(self.types.register_type(Type::ByteArray { width }))
                } else {
                    data_location.map(|data_location| {
                        self.types.register_type(Type::Bytes {
                            location: data_location,
                        })
                    })
                }
            }
            input_ir::ElementaryType::IntKeyword(int_keyword) => {
                let bits = int_keyword
                    .unparse()
                    .strip_prefix("int")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Some(
                    self.types
                        .register_type(Type::Integer { signed: true, bits }),
                )
            }
            input_ir::ElementaryType::UintKeyword(uint_keyword) => {
                let bits = uint_keyword
                    .unparse()
                    .strip_prefix("uint")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(256);
                Some(self.types.register_type(Type::Integer {
                    signed: false,
                    bits,
                }))
            }
            input_ir::ElementaryType::FixedKeyword(fixed_keyword) => {
                let fixed_keyword = fixed_keyword.unparse();
                let mut parts = fixed_keyword
                    .strip_prefix("fixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap());
                let bits = parts.next().unwrap();
                let precision_bits = parts.next().unwrap_or(0);
                Some(self.types.register_type(Type::FixedPointNumber {
                    signed: true,
                    bits,
                    precision_bits,
                }))
            }
            input_ir::ElementaryType::UfixedKeyword(ufixed_keyword) => {
                let ufixed_keyword = ufixed_keyword.unparse();
                let mut parts = ufixed_keyword
                    .strip_prefix("ufixed")
                    .unwrap()
                    .split('x')
                    .map(|part| part.parse::<u32>().unwrap());
                let bits = parts.next().unwrap();
                let precision_bits = parts.next().unwrap_or(0);
                Some(self.types.register_type(Type::FixedPointNumber {
                    signed: false,
                    bits,
                    precision_bits,
                }))
            }
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
                input_ir::FunctionAttribute::ExternalKeyword => external = true,
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
        self.resolve_inheritance_types(&node.inheritance_types);
        // TODO: save the resolved types as bases of the contract
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_meta_type_node(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        if let Some(inheritance) = &node.inheritance {
            self.resolve_inheritance_types(&inheritance.types);
            // TODO: save the resolved types as bases of the interface
        }
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_meta_type_node(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);

        self.binder.mark_meta_type_node(node.node_id);
    }

    fn leave_path_import(&mut self, node: &input_ir::PathImport) {
        if node.alias.is_some() {
            self.binder.mark_meta_type_node(node.node_id);
        }
    }

    fn leave_named_import(&mut self, node: &input_ir::NamedImport) {
        self.binder.mark_meta_type_node(node.node_id);
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        let type_id = self.type_of_function_definition(node);
        self.binder.set_node_type(node.node_id, type_id);
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

    fn leave_parameter(&mut self, node: &input_ir::Parameter) {
        let type_id = self.resolve_type_name(
            &node.type_name,
            node.storage_location
                .as_ref()
                .map(storage_location_to_data_location),
        );
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_event_definition(&mut self, node: &input_ir::EventDefinition) {
        self.binder.mark_meta_type_node(node.node_id);
    }

    fn leave_event_parameter(&mut self, node: &input_ir::EventParameter) {
        // TODO: the data location is not strictly correct, but strings, bytes
        // and structs are allowed as event parameters and they won't type if we
        // pass None here
        let type_id = self.resolve_type_name(&node.type_name, Some(DataLocation::Memory));
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_error_definition(&mut self, node: &input_ir::ErrorDefinition) {
        self.binder.mark_meta_type_node(node.node_id);
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
                        .map(storage_location_to_data_location),
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
                .map(storage_location_to_data_location),
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
        self.binder.mark_meta_type_node(node.node_id);
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

        self.binder.mark_meta_type_node(node.node_id);
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        node: &input_ir::UserDefinedValueTypeDefinition,
    ) {
        let type_id = self.type_of_elementary_type(&node.value_type, None);
        self.binder.set_node_type(node.node_id, type_id);
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
}
