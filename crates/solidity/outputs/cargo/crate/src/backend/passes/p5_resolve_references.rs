use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use semver::Version;

use super::p4_type_definitions::Output as Input;
use crate::backend::binder::{
    Binder, Definition, EitherIter, Reference, Resolution, ResolveOptions, Scope, ScopeId, Typing,
    UsingDirective,
};
use crate::backend::built_ins::BuiltInsResolver;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, FunctionType, Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

/// This pass will find identifiers used as references, resolve them to the
/// appropriate definitions, and compute typing information for AST nodes
/// containing expressions and statements. Both these actions are co-dependant
/// and happen concurrently for each node, and their results are store in the
/// `Binder` instance.
pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(
        input.binder,
        input.types,
        compilation_unit.language_version(),
    );
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

const VERSION_0_5_0: Version = Version::new(0, 5, 0);
const VERSION_0_7_0: Version = Version::new(0, 7, 0);

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder, types: TypeRegistry, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types,
        }
    }

    fn built_ins_resolver(&self) -> BuiltInsResolver<'_> {
        BuiltInsResolver::new(self.language_version.clone(), &self.binder, &self.types)
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

    fn current_scope_id(&self) -> ScopeId {
        self.scope_stack.last().copied().unwrap()
    }

    // This is a "top-level" (ie. not a member access) resolution method
    fn resolve_symbol_in_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        match &resolution {
            Resolution::Unresolved => self.built_ins_resolver().lookup_global(symbol).into(),
            Resolution::Ambiguous(definition_ids) => {
                // Try to disambiguate known cases
                let first_id = definition_ids.first().copied().unwrap();
                let first_definition = self.binder.find_definition_by_id(first_id).unwrap();

                match first_definition {
                    Definition::StateVariable(_) => {
                        // TODO(validation): the state variable should have the
                        // `override` attribute and the rest of the definitions
                        // should be either functions with the correct
                        // signature, state variables or private variables or
                        // constants
                        Resolution::Definition(first_id)
                    }
                    Definition::Constant(_) => {
                        // TODO(validation): if there are other definitions in
                        // base contracts, they should be marked private and
                        // they should be constants or state variables
                        Resolution::Definition(first_id)
                    }
                    _ => {
                        // TODO(validation): check that the returned definitions are
                        // all functions (or maybe modifiers?)
                        resolution
                    }
                }
            }
            Resolution::Definition(_) | Resolution::BuiltIn(_) => resolution,
        }
    }

    fn active_using_directives_for_type(
        &self,
        type_id: TypeId,
    ) -> impl Iterator<Item = &UsingDirective> {
        // Compute the canonical type: this handles cases where the given type
        // is context dependent:
        // - If the type is a reference type, we need to compute the id of the
        // location independent type (using DataLocation::Inherited). If it
        // doesn't exist, proceed with the given value, but we won't find any
        // type-specific directives, only those applicable to all types (ie.
        // `using L for *`)
        // - If the type is a function type, it may have an associated
        // definition ID from the function definition where it is derived from.
        let type_id = self
            .types
            .find_canonical_type_id(type_id)
            .unwrap_or(type_id);

        // consider using directives in the scope stack
        self.scope_stack
            .iter()
            .rev()
            .flat_map(|scope_id| {
                if self.language_version < VERSION_0_7_0 {
                    // In Solidity < 0.7.0 using directives are inherited in contracts,
                    // so we need to pull any `using` directives in a contract hierarchy
                    // if there are linearisations
                    EitherIter::Left(
                        self.binder
                            .get_using_directives_in_scope_including_inherited(*scope_id),
                    )
                } else {
                    EitherIter::Right(self.binder.get_using_directives_in_scope(*scope_id))
                }
            })
            // ... and add the global directives
            .chain(self.binder.get_global_using_directives())
            .filter(move |directive| directive.applies_to(type_id))
    }

    fn resolve_symbol_in_typing(&self, typing: &Typing, symbol: &str) -> Resolution {
        match typing {
            Typing::Unresolved => Resolution::Unresolved,
            Typing::Undetermined(type_ids) => {
                // We cannot use argument-type disambiguation here, so we will
                // use the first result
                // TODO(validation): check that the types are consistent (eg.
                // they are all function types) and that it makes sense to use
                // the first one
                self.resolve_symbol_in_type(type_ids[0], symbol)
            }
            Typing::Resolved(type_id) => self.resolve_symbol_in_type(*type_id, symbol),
            Typing::This | Typing::Super => {
                // TODO: the contract scope here is not necessarily the current
                // lexical scope; for compilation we should set it to the scope
                // of the contract being compiled, as this will affect the
                // linearisation and hence the result of this `super`
                // resolution. This affects the first parameter to
                // `resolve_in_contract_scope`, not the `node_id` of the
                // resolution option which is always lexical.
                if let Some(scope_id) = self.current_contract_scope_id() {
                    let node_id = self.binder.get_scope_by_id(scope_id).node_id();
                    let options = if matches!(typing, Typing::This) {
                        ResolveOptions::This(node_id)
                    } else {
                        ResolveOptions::Super(node_id)
                    };
                    // TODO(validation): for `this` resolutions we need to check
                    // that the returned definitions are externally available
                    // (ie. either `external` or `public`)
                    let resolution = self
                        .binder
                        .resolve_in_contract_scope(scope_id, symbol, options);

                    if self.language_version < VERSION_0_5_0 && resolution == Resolution::Unresolved
                    {
                        // In Solidity < 0.5.0, `this` can be used as an address
                        Resolution::from(
                            self.built_ins_resolver()
                                .lookup_member_of_type_id(self.types.address(), symbol),
                        )
                    } else {
                        resolution
                    }
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::NewExpression(type_id) => {
                // Special case: resolve legacy constructor call options (ie.
                // `(new Lock).value(1)()`)
                if self.language_version < VERSION_0_7_0 {
                    Resolution::from(
                        self.built_ins_resolver()
                            .lookup_member_of_new_type_id(*type_id, symbol),
                    )
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::MetaType(type_) => {
                if let Some(built_in) = self
                    .built_ins_resolver()
                    .lookup_member_of_meta_type(type_, symbol)
                {
                    Resolution::BuiltIn(built_in)
                } else {
                    Resolution::Unresolved
                }
            }
            Typing::UserMetaType(node_id) => {
                // We're trying to resolve a member access expression into a
                // type name, ie. this is a meta-type member access
                let Some(definition) = self.binder.find_definition_by_id(*node_id) else {
                    return Resolution::Unresolved;
                };
                match definition {
                    Definition::Import(import_definition) => {
                        if let Some(scope_id) = import_definition
                            .resolved_file_id
                            .as_ref()
                            .and_then(|file_id| self.binder.scope_id_for_file_id(file_id))
                        {
                            self.binder.resolve_in_scope(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    Definition::Contract(_)
                    | Definition::Enum(_)
                    | Definition::Interface(_)
                    | Definition::Library(_) => {
                        // this is a "namespace" lookup
                        if let Some(scope_id) = self.binder.scope_id_for_node_id(*node_id) {
                            self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                        } else {
                            Resolution::Unresolved
                        }
                    }
                    _ => self
                        .built_ins_resolver()
                        .lookup_member_of_user_definition(definition, symbol)
                        .into(),
                }
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .lookup_member_of(built_in, symbol)
                .into(),
        }
    }

    fn resolve_symbol_in_type(&self, type_id: TypeId, symbol: &str) -> Resolution {
        let type_ = self.types.get_type_by_id(type_id);
        match type_ {
            Type::Contract { definition_id, .. } | Type::Interface { definition_id, .. } => {
                // TODO(validation): check that the found definitions are public
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder
                    .resolve_in_contract_scope(scope_id, symbol, ResolveOptions::Normal)
                    .or_else(|| {
                        self.built_ins_resolver()
                            .lookup_member_of_type_id(type_id, symbol)
                            .into()
                    })
            }
            Type::Struct { definition_id, .. } => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
            _ => self
                .built_ins_resolver()
                .lookup_member_of_type_id(type_id, symbol)
                .into(),
        }
        .or_else(|| {
            // Consider active `using` directives in the current context
            let active_directives = self.active_using_directives_for_type(type_id);
            let mut definition_ids = Vec::new();
            let mut seen_ids = HashSet::new();
            for directive in active_directives {
                let scope_id = directive.get_scope_id();
                let ids = self
                    .binder
                    .resolve_in_scope_as_namespace(scope_id, symbol)
                    .get_definition_ids();
                // TODO: filter the resolved definitions to only include
                // functions whose first parameter is of our type (or
                // implicitly convertible to it)
                for id in &ids {
                    // Avoid returning duplicate definition IDs. That may happen
                    // if equivalent `using` directives are active at some point
                    // (eg. inherited through a base in older Solidity)
                    if seen_ids.insert(*id) {
                        definition_ids.push(*id);
                    }
                }
            }
            Resolution::from(definition_ids)
        })
    }

    fn typing_of_expression(&self, node: &input_ir::Expression) -> Typing {
        match node {
            input_ir::Expression::AssignmentExpression(assignment_expression) => {
                self.binder.node_typing(assignment_expression.node_id)
            }
            input_ir::Expression::ConditionalExpression(conditional_expression) => {
                self.binder.node_typing(conditional_expression.node_id)
            }
            input_ir::Expression::OrExpression(_)
            | input_ir::Expression::AndExpression(_)
            | input_ir::Expression::EqualityExpression(_)
            | input_ir::Expression::InequalityExpression(_)
            | input_ir::Expression::TrueKeyword
            | input_ir::Expression::FalseKeyword => Typing::Resolved(self.types.boolean()),
            input_ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.binder.node_typing(bitwise_or_expression.node_id)
            }
            input_ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.binder.node_typing(bitwise_xor_expression.node_id)
            }
            input_ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.binder.node_typing(bitwise_and_expression.node_id)
            }
            input_ir::Expression::ShiftExpression(shift_expression) => {
                self.binder.node_typing(shift_expression.node_id)
            }
            input_ir::Expression::AdditiveExpression(additive_expression) => {
                self.binder.node_typing(additive_expression.node_id)
            }
            input_ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.binder.node_typing(multiplicative_expression.node_id)
            }
            input_ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.binder.node_typing(exponentiation_expression.node_id)
            }
            input_ir::Expression::PostfixExpression(postfix_expression) => {
                self.binder.node_typing(postfix_expression.node_id)
            }
            input_ir::Expression::PrefixExpression(prefix_expression) => {
                self.binder.node_typing(prefix_expression.node_id)
            }
            input_ir::Expression::FunctionCallExpression(function_call_expression) => {
                self.binder.node_typing(function_call_expression.node_id)
            }
            input_ir::Expression::CallOptionsExpression(call_options_expression) => {
                self.binder.node_typing(call_options_expression.node_id)
            }
            input_ir::Expression::MemberAccessExpression(member_access_expression) => {
                self.binder.node_typing(member_access_expression.node_id)
            }
            input_ir::Expression::IndexAccessExpression(index_access_expression) => {
                self.binder.node_typing(index_access_expression.node_id)
            }
            input_ir::Expression::NewExpression(new_expression) => {
                self.binder.node_typing(new_expression.node_id)
            }
            input_ir::Expression::TupleExpression(tuple_expression) => {
                self.binder.node_typing(tuple_expression.node_id)
            }
            input_ir::Expression::TypeExpression(type_expression) => {
                self.binder.node_typing(type_expression.node_id)
            }
            input_ir::Expression::ArrayExpression(array_expression) => {
                self.binder.node_typing(array_expression.node_id)
            }
            input_ir::Expression::HexNumberExpression(hex_number_expression) => {
                if Self::hex_number_is_address_literal(hex_number_expression) {
                    Typing::Resolved(self.types.address())
                } else {
                    Typing::Resolved(self.types.rational())
                }
            }
            input_ir::Expression::DecimalNumberExpression(_) => {
                Typing::Resolved(self.types.rational())
            }
            input_ir::Expression::StringExpression(_) => Typing::Resolved(self.types.string()),
            input_ir::Expression::ElementaryType(elementary_type) => {
                Typing::MetaType(Self::type_of_elementary_type(elementary_type))
            }
            input_ir::Expression::Identifier(identifier) => self.typing_of_identifier(identifier),
            input_ir::Expression::PayableKeyword => {
                Typing::MetaType(Type::Address { payable: true })
            }
            input_ir::Expression::ThisKeyword => Typing::This,
            input_ir::Expression::SuperKeyword => Typing::Super,
        }
    }

    fn hex_number_is_address_literal(
        hex_number_expression: &input_ir::HexNumberExpression,
    ) -> bool {
        if hex_number_expression.unit.is_some() {
            return false;
        }
        let hex_number = hex_number_expression.literal.unparse();
        // TODO(validation): verify the address is valid (ie. has a valid checksum)
        // We need at least an implementation of SHA3 to compute the checksum
        hex_number.len() == 42
    }

    fn type_of_elementary_type(elementary_type: &input_ir::ElementaryType) -> Type {
        match elementary_type {
            input_ir::ElementaryType::AddressType(address_type) => Type::Address {
                payable: address_type.payable_keyword.is_some(),
            },
            input_ir::ElementaryType::BytesKeyword(terminal) => {
                Type::from_bytes_keyword(&terminal.unparse(), Some(DataLocation::Inherited))
                    .unwrap()
            }
            input_ir::ElementaryType::IntKeyword(terminal) => {
                Type::from_int_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::UintKeyword(terminal) => {
                Type::from_uint_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::FixedKeyword(terminal) => {
                Type::from_fixed_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::UfixedKeyword(terminal) => {
                Type::from_ufixed_keyword(&terminal.unparse())
            }
            input_ir::ElementaryType::BoolKeyword => Type::Boolean,
            input_ir::ElementaryType::ByteKeyword => Type::ByteArray { width: 1 },
            input_ir::ElementaryType::StringKeyword => Type::String {
                location: DataLocation::Inherited,
            },
        }
    }

    fn typing_of_identifier(&self, identifier: &Rc<TerminalNode>) -> Typing {
        let resolution = self
            .binder
            .find_reference_by_identifier_node_id(identifier.id())
            .unwrap()
            .resolution
            .clone();
        self.typing_of_resolution(&resolution)
    }

    fn type_of_integer_binary_expression(
        &self,
        left_operand: &input_ir::Expression,
        right_operand: &input_ir::Expression,
    ) -> Option<TypeId> {
        let left_type_id = self.typing_of_expression(left_operand).as_type_id();
        let right_type_id = self.typing_of_expression(right_operand).as_type_id();
        // TODO(validation): check that both operands are indeed integers
        if let (Some(left_type_id), Some(right_type_id)) = (left_type_id, right_type_id) {
            if self
                .types
                .implicitly_convertible_to(right_type_id, left_type_id)
            {
                Some(left_type_id)
            } else if self
                .types
                .implicitly_convertible_to(left_type_id, right_type_id)
            {
                Some(right_type_id)
            } else {
                // TODO(validation): the types are not compatible, we should
                // emit an error, or signal our caller
                None
            }
        } else {
            None
        }
    }

    fn typing_of_resolution(&self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Unresolved => Typing::Unresolved,
            Resolution::BuiltIn(built_in) => self.built_ins_resolver().typing_of(built_in),
            Resolution::Definition(definition_id) => self.binder.node_typing(*definition_id),
            Resolution::Ambiguous(definitions) => {
                let mut type_ids = Vec::new();
                for definition_id in definitions {
                    if let Typing::Resolved(type_id) = self.binder.node_typing(*definition_id) {
                        type_ids.push(type_id);
                    }
                }
                Typing::Undetermined(type_ids)
            }
        }
    }

    fn current_contract_scope_id(&self) -> Option<ScopeId> {
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            if matches!(scope, Scope::Contract(_)) {
                return Some(*scope_id);
            }
        }
        None
    }

    fn resolve_modifier_invocation(&mut self, modifier_invocation: &input_ir::ModifierInvocation) {
        let identifier_path = &modifier_invocation.name;
        let mut scope_id = self.current_contract_scope_id();
        let mut use_contract_lookup = true;
        for identifier in identifier_path {
            let resolution = if let Some(scope_id) = scope_id {
                let symbol = identifier.unparse();
                if use_contract_lookup {
                    use_contract_lookup = false;
                    self.resolve_symbol_in_scope(scope_id, &symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, &symbol)
                }
            } else {
                Resolution::Unresolved
            };
            // TODO(validation): the found definition(s) must be modifiers
            // and be in the current contract hierarchy. We could potentially
            // verify that the initial symbol lookup is reachable from the
            // contract only (ie. it's a contract modifier, a modifier in a
            // base, or it's the identifier of a base of the current contract)

            scope_id = resolution
                .as_definition_id()
                .and_then(|definition_id| self.binder.scope_id_for_node_id(definition_id));

            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }

    fn resolve_named_arguments(
        &mut self,
        named_arguments: &[input_ir::NamedArgument],
        definition_id: Option<NodeId>,
    ) {
        let parameters_scope_id = definition_id.and_then(|definition_id| {
            self.binder
                .get_parameters_scope_for_definition(definition_id)
        });

        for named_argument in named_arguments {
            let identifier = &named_argument.name;
            let resolution =
                parameters_scope_id.map_or(Resolution::Unresolved, |parameters_scope_id| {
                    self.binder
                        .resolve_in_scope_as_namespace(parameters_scope_id, &identifier.unparse())
                });
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
    }

    // This is a "top-level" resolution method for symbols in s Yul context
    fn resolve_symbol_in_yul_scope(&self, scope_id: ScopeId, symbol: &str) -> Resolution {
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        if resolution == Resolution::Unresolved {
            self.built_ins_resolver().lookup_yul_global(symbol).into()
        } else {
            resolution
        }
    }

    fn resolve_yul_suffix(&self, symbol: &str, parent_resolution: &Resolution) -> Resolution {
        match parent_resolution {
            Resolution::Definition(node_id) => {
                if let Some(definition) = self.binder.find_definition_by_id(*node_id) {
                    self.built_ins_resolver()
                        .lookup_yul_suffix(definition, symbol)
                        .into()
                } else {
                    Resolution::Unresolved
                }
            }
            Resolution::Unresolved | Resolution::Ambiguous(_) | Resolution::BuiltIn(_) => {
                Resolution::Unresolved
            }
        }
    }

    fn typing_is_contract_reference(&self, typing: &Typing) -> bool {
        match typing {
            Typing::This => true,
            Typing::Resolved(type_id) => matches!(
                self.types.get_type_by_id(*type_id),
                Type::Contract { .. } | Type::Interface { .. }
            ),
            _ => false,
        }
    }

    fn typing_of_resolution_as_getter(&self, resolution: &Resolution) -> Typing {
        if let Resolution::Definition(definition_id) = resolution {
            if let Definition::StateVariable(state_var_definition) =
                self.binder.find_definition_by_id(*definition_id).unwrap()
            {
                if let Some(getter_type_id) = state_var_definition.getter_type_id {
                    return Typing::Resolved(getter_type_id);
                }
            }
        }
        self.typing_of_resolution(resolution)
    }

    fn lookup_function_matching_positional_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[Typing],
        receiver_type_id: Option<TypeId>,
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let parameter_types = &function_type.parameter_types;
            if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                self.matches_positional_arguments(parameter_types, argument_typings)
            } else if let Some(receiver_type_id) = receiver_type_id {
                // we have a receiver type, so check the first parameter type
                // against it and then the rest, if the counts match
                if parameter_types.len() == argument_typings.len() + 1
                    && parameter_types.first().is_some_and(|type_id| {
                        self.types
                            .implicitly_convertible_to(receiver_type_id, *type_id)
                    })
                {
                    self.matches_positional_arguments(&parameter_types[1..], argument_typings)
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn matches_positional_arguments(
        &self,
        parameter_types: &[TypeId],
        argument_typings: &[Typing],
    ) -> bool {
        parameter_types
            .iter()
            .zip(argument_typings)
            .all(|(parameter_type, argument_typing)| {
                self.parameter_type_matches_argument_typing(*parameter_type, argument_typing)
            })
    }

    fn parameter_type_matches_argument_typing(
        &self,
        parameter_type: TypeId,
        argument_typing: &Typing,
    ) -> bool {
        match argument_typing {
            Typing::Resolved(type_id) => self
                .types
                .implicitly_convertible_to(*type_id, parameter_type),
            Typing::This => self
                .types
                .implicitly_convertible_to(self.types.address(), parameter_type),
            _ => false,
        }
    }

    fn type_id_of_receiver(&self, operand: &input_ir::Expression) -> Option<TypeId> {
        if let input_ir::Expression::MemberAccessExpression(member_access_expression) = operand {
            self.typing_of_expression(&member_access_expression.operand)
                .as_type_id()
        } else {
            None
        }
    }

    fn typing_of_function_call_with_positional_arguments(
        &mut self,
        node: &input_ir::FunctionCallExpression,
        arguments: &[input_ir::Expression],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);
        let argument_typings = arguments
            .iter()
            .map(|argument| self.typing_of_expression(argument))
            .collect::<Vec<_>>();

        match operand_typing {
            Typing::Unresolved | Typing::This | Typing::Super => {
                // `this` and `super` are not callable
                Typing::Unresolved
            }
            Typing::Resolved(type_id) => {
                if let Type::Function(FunctionType { return_type, .. }) =
                    self.types.get_type_by_id(type_id)
                {
                    Typing::Resolved(*return_type)
                } else {
                    // TODO(validation): the operand did not resolve to a function
                    Typing::Unresolved
                }
            }
            Typing::Undetermined(type_ids) => {
                // TODO: maybe update the typing of the operand as well?
                let receiver_type_id = self.type_id_of_receiver(&node.operand);
                let candidate = self.lookup_function_matching_positional_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    Typing::Resolved(return_type)
                } else {
                    Typing::Unresolved
                }
            }
            Typing::MetaType(type_) => {
                // TODO(validation): this is a cast to the given type, but we
                // need to verify that the (single) argument is convertible
                if let Some(typing_of_first_argument) = argument_typings.first().cloned() {
                    match typing_of_first_argument {
                        Typing::Resolved(type_id) => {
                            let data_location = self.types.get_type_by_id(type_id).data_location();
                            let type_id = self
                                .types
                                .register_type(type_.with_data_location(data_location));
                            Typing::Resolved(type_id)
                        }
                        Typing::This => {
                            // special case: "this" can be cast to an address
                            if let Type::Address { .. } = type_ {
                                Typing::Resolved(self.types.address())
                            } else {
                                Typing::Unresolved
                            }
                        }
                        _ => Typing::Unresolved,
                    }
                } else {
                    Typing::Unresolved
                }
            }
            Typing::NewExpression(type_id) => {
                if let Type::Contract { .. } = self.types.get_type_by_id(type_id) {
                    Typing::Resolved(type_id)
                } else {
                    // only contracts can be created with `new`
                    Typing::Unresolved
                }
            }
            Typing::UserMetaType(node_id) => {
                // Generally this is a cast to the underlying type of the given
                // definition, except for structs for which we need to construct
                // the value in memory
                match self.binder.find_definition_by_id(node_id) {
                    Some(Definition::Contract(_)) => {
                        // TODO(validation): the type of the first argument should be an address
                        let type_id = self.types.register_type(Type::Contract {
                            definition_id: node_id,
                        });
                        Typing::Resolved(type_id)
                    }
                    Some(Definition::Interface(_)) => {
                        // TODO(validation): the type of the first argument should be an address
                        let type_id = self.types.register_type(Type::Interface {
                            definition_id: node_id,
                        });
                        Typing::Resolved(type_id)
                    }
                    Some(Definition::Struct(_)) => {
                        // struct construction
                        let type_id = self.types.register_type(Type::Struct {
                            definition_id: node_id,
                            location: DataLocation::Memory,
                        });
                        Typing::Resolved(type_id)
                    }
                    _ => Typing::Unresolved,
                }
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .typing_of_function_call(&built_in, &argument_typings),
        }
    }

    fn lookup_function_matching_named_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[(String, Typing)],
        receiver_type_id: Option<TypeId>,
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let Some(definition_id) = function_type.definition_id else {
                return false;
            };
            let definition = self.binder.find_definition_by_id(definition_id).unwrap();
            let Definition::Function(function_definition) = definition else {
                unreachable!("the definition associated to a function type is not a function");
            };
            let parameter_types = &function_type.parameter_types;
            let parameter_names = &function_definition.parameter_names;
            assert_eq!(
                parameter_types.len(),
                parameter_names.len(),
                "length of types and names of parameters should match"
            );
            if parameter_names.iter().any(|name| name.is_none()) {
                // function has an unnamed parameter and we cannot use it for
                // matching named arguments
                return false;
            }

            if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                self.matches_named_arguments(parameter_names, parameter_types, argument_typings)
            } else if let Some(receiver_type_id) = receiver_type_id {
                // we have a receiver type, so check the first parameter type
                // against it and then the rest, if the counts match
                if parameter_types.len() == argument_typings.len() + 1
                    && parameter_types.first().is_some_and(|type_id| {
                        self.types
                            .implicitly_convertible_to(receiver_type_id, *type_id)
                    })
                {
                    self.matches_named_arguments(
                        &parameter_names[1..],
                        &parameter_types[1..],
                        argument_typings,
                    )
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn matches_named_arguments(
        &self,
        parameter_names: &[Option<String>],
        parameter_types: &[TypeId],
        argument_typings: &[(String, Typing)],
    ) -> bool {
        argument_typings
            .iter()
            .all(|(argument_name, argument_typing)| {
                let Some(index) = parameter_names
                    .iter()
                    .position(|name| name.as_ref().is_some_and(|name| name == argument_name))
                else {
                    return false;
                };
                let parameter_type = parameter_types[index];
                self.parameter_type_matches_argument_typing(parameter_type, argument_typing)
            })
    }

    fn typing_of_function_call_with_named_arguments(
        &mut self,
        node: &input_ir::FunctionCallExpression,
        arguments: &[input_ir::NamedArgument],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);

        let (typing, definition_id) = match operand_typing {
            Typing::Unresolved | Typing::This | Typing::Super => {
                // `this` and `super` are not callable
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => {
                if let Type::Function(FunctionType {
                    definition_id,
                    return_type,
                    ..
                }) = self.types.get_type_by_id(type_id)
                {
                    (Typing::Resolved(*return_type), *definition_id)
                } else {
                    // TODO(validation): the operand did not resolve to a function
                    (Typing::Unresolved, None)
                }
            }
            Typing::Undetermined(type_ids) => {
                // TODO: resolve argument types and match best overload
                // TODO: maybe update the typing of the operand?
                // TODO: return the definition_id used to later resolve named arguments
                let receiver_type_id = self.type_id_of_receiver(&node.operand);
                let argument_typings = arguments
                    .iter()
                    .map(|argument| {
                        (
                            argument.name.unparse(),
                            self.typing_of_expression(&argument.value),
                        )
                    })
                    .collect::<Vec<_>>();
                let candidate = self.lookup_function_matching_named_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    (Typing::Resolved(return_type), definition_id)
                } else {
                    (Typing::Unresolved, None)
                }
            }
            Typing::MetaType(_) => {
                // This is a cast to the given type and is not valid with named arguments
                (Typing::Unresolved, None)
            }
            Typing::NewExpression(type_id) => {
                if let Type::Contract { definition_id } = self.types.get_type_by_id(type_id) {
                    (Typing::Resolved(type_id), Some(*definition_id))
                } else {
                    // only contracts can be created with `new`
                    (Typing::Unresolved, None)
                }
            }
            Typing::UserMetaType(node_id) => {
                // Function call with named arguments are only valid in user
                // types of the struct kind, which results in the construction
                // of such struct in memory
                match self.binder.find_definition_by_id(node_id) {
                    Some(Definition::Struct(_)) => {
                        // struct construction
                        let type_id = self.types.register_type(Type::Struct {
                            definition_id: node_id,
                            location: DataLocation::Memory,
                        });
                        (Typing::Resolved(type_id), Some(node_id))
                    }
                    _ => (Typing::Unresolved, None),
                }
            }
            Typing::BuiltIn(_) => {
                // built-ins cannot be called with named arguments
                (Typing::Unresolved, None)
            }
        };

        // Reference and resolve named arguments
        self.resolve_named_arguments(arguments, definition_id);

        typing
    }
}

// Given an expression node that resolves to a reference, return the node ID of
// the identifier of the reference. If the expression cannot be traced back to a
// single reference, return `None`.
fn reference_node_id_for_expression(node: &input_ir::Expression) -> Option<NodeId> {
    match &node {
        input_ir::Expression::MemberAccessExpression(f) => Some(f.member.id()),
        input_ir::Expression::Identifier(f) => Some(f.id()),
        input_ir::Expression::CallOptionsExpression(f) => {
            reference_node_id_for_expression(&f.operand)
        }
        _ => None,
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
        // Push the contract scope to visit the contract members
        self.enter_scope_for_node_id(node.node_id);
        for member in &node.members {
            input_ir::visitor::accept_contract_member(member, self);
        }
        self.leave_scope_for_node_id(node.node_id);

        // But any reference in the inheritance types and the storage layout
        // specifier should resolve in the parent scope
        for inheritance_type in &node.inheritance_types {
            input_ir::visitor::accept_inheritance_type(inheritance_type, self);
        }
        if let Some(ref storage_layout) = node.storage_layout {
            input_ir::visitor::accept_storage_layout_specifier(storage_layout, self);
        }

        // We already visited the contract definition's children above to be
        // able to better control the scope stack, so don't recurse again
        false
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        for attribute in &node.attributes {
            if let input_ir::FunctionAttribute::ModifierInvocation(modifier_invocation) = attribute
            {
                self.resolve_modifier_invocation(modifier_invocation);
            }
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        // TODO(validation): modifiers are not allowed inside interfaces since 0.8.8
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) -> bool {
        for attribute in &node.attributes {
            if let input_ir::ConstructorAttribute::ModifierInvocation(modifier_invocation) =
                attribute
            {
                self.resolve_modifier_invocation(modifier_invocation);
            }
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_fallback_function_definition(
        &mut self,
        node: &input_ir::FallbackFunctionDefinition,
    ) -> bool {
        for attribute in &node.attributes {
            if let input_ir::FallbackFunctionAttribute::ModifierInvocation(modifier_invocation) =
                attribute
            {
                self.resolve_modifier_invocation(modifier_invocation);
            }
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_fallback_function_definition(&mut self, node: &input_ir::FallbackFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_receive_function_definition(
        &mut self,
        node: &input_ir::ReceiveFunctionDefinition,
    ) -> bool {
        for attribute in &node.attributes {
            if let input_ir::ReceiveFunctionAttribute::ModifierInvocation(modifier_invocation) =
                attribute
            {
                self.resolve_modifier_invocation(modifier_invocation);
            }
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_receive_function_definition(&mut self, node: &input_ir::ReceiveFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_unnamed_function_definition(
        &mut self,
        node: &input_ir::UnnamedFunctionDefinition,
    ) -> bool {
        for attribute in &node.attributes {
            if let input_ir::UnnamedFunctionAttribute::ModifierInvocation(modifier_invocation) =
                attribute
            {
                self.resolve_modifier_invocation(modifier_invocation);
            }
        }

        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_unnamed_function_definition(&mut self, node: &input_ir::UnnamedFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_block(&mut self, node: &input_ir::Block) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_block(&mut self, node: &input_ir::Block) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_for_statement(&mut self, node: &input_ir::ForStatement) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_for_statement(&mut self, node: &input_ir::ForStatement) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_yul_block(&mut self, node: &input_ir::YulBlock) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_block(&mut self, node: &input_ir::YulBlock) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_expression(&mut self, node: &input_ir::Expression) -> bool {
        if let input_ir::Expression::Identifier(identifier) = node {
            let scope_id = self.current_scope_id();
            let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_assignment_expression(&mut self, node: &input_ir::AssignmentExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the type of right_operand can be applied
        // to the left by means of the operator
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_conditional_expression(&mut self, node: &input_ir::ConditionalExpression) {
        let type_id = self
            .typing_of_expression(&node.true_expression)
            .as_type_id();
        // TODO(validation): both true_expression and false_expression should
        // have the same type
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_or_expression(&mut self, _node: &input_ir::OrExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_and_expression(&mut self, _node: &input_ir::AndExpression) {
        // TODO(validation): check that both operands are boolean
    }

    fn leave_equality_expression(&mut self, _node: &input_ir::EqualityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_inequality_expression(&mut self, _node: &input_ir::InequalityExpression) {
        // TODO(validation): check that both operands have a compatible type
    }

    fn leave_bitwise_or_expression(&mut self, node: &input_ir::BitwiseOrExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_xor_expression(&mut self, node: &input_ir::BitwiseXorExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_bitwise_and_expression(&mut self, node: &input_ir::BitwiseAndExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_shift_expression(&mut self, node: &input_ir::ShiftExpression) {
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_additive_expression(&mut self, node: &input_ir::AdditiveExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_multiplicative_expression(&mut self, node: &input_ir::MultiplicativeExpression) {
        let type_id =
            self.type_of_integer_binary_expression(&node.left_operand, &node.right_operand);
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_exponentiation_expression(&mut self, node: &input_ir::ExponentiationExpression) {
        let mut type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
        if type_id.is_some_and(|type_id| type_id == self.types.rational()) {
            // if the base is a rational but the exponent is not, then the result is uint256
            if self
                .typing_of_expression(&node.right_operand)
                .as_type_id()
                .is_none_or(|exponent_type| exponent_type != self.types.rational())
            {
                type_id = Some(self.types.uint256());
            }
        }
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_postfix_expression(&mut self, node: &input_ir::PostfixExpression) {
        // TODO(validation): check that the operand is an integer
        let type_id = self.typing_of_expression(&node.operand).as_type_id();
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_prefix_expression(&mut self, node: &input_ir::PrefixExpression) {
        let type_id = match node.operator.kind {
            TerminalKind::PlusPlus
            | TerminalKind::Plus
            | TerminalKind::MinusMinus
            | TerminalKind::Minus
            | TerminalKind::Tilde => {
                // TODO(validation): check that the operand is integer
                self.typing_of_expression(&node.operand).as_type_id()
            }
            TerminalKind::Bang => {
                // TODO(validation): check that the operand is boolean
                Some(self.types.boolean())
            }
            TerminalKind::DeleteKeyword => Some(self.types.void()),
            _ => None,
        };
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_tuple_expression(&mut self, node: &input_ir::TupleExpression) {
        let typing = if node.items.len() == 1 {
            node.items
                .first()
                .unwrap()
                .expression
                .as_ref()
                .map_or(Typing::Unresolved, |expression| {
                    self.typing_of_expression(expression)
                })
        } else {
            let mut types = Vec::new();
            for item in &node.items {
                let type_id = item
                    .expression
                    .as_ref()
                    .and_then(|expression| self.typing_of_expression(expression).as_type_id())
                    .unwrap_or(self.types.void());
                types.push(type_id);
            }
            let type_id = self.types.register_type(Type::Tuple { types });
            Typing::Resolved(type_id)
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let resolution = self.resolve_symbol_in_typing(&operand_typing, &node.member.unparse());

        // Special case: if the operand is either `this` or a contract/interface
        // reference type, then try to type the member as a getter
        let typing = if self.typing_is_contract_reference(&operand_typing) {
            self.typing_of_resolution_as_getter(&resolution)
        } else {
            self.typing_of_resolution(&resolution)
        };
        self.binder.set_node_typing(node.node_id, typing);

        let reference = Reference::new(Rc::clone(&node.member), resolution);
        self.binder.insert_reference(reference);
    }

    fn leave_index_access_expression(&mut self, node: &input_ir::IndexAccessExpression) {
        let typing =
            if let Some(operand_type_id) = self.typing_of_expression(&node.operand).as_type_id() {
                let operand_type = self.types.get_type_by_id(operand_type_id);
                match operand_type {
                    Type::Array { element_type, .. } => Typing::Resolved(*element_type),
                    Type::Mapping { value_type_id, .. } => Typing::Resolved(*value_type_id),
                    _ => {
                        // TODO(validation): the operand is not indexable
                        Typing::Unresolved
                    }
                }
            } else {
                Typing::Unresolved
            };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_array_expression(&mut self, node: &input_ir::ArrayExpression) {
        let typing = if node.items.is_empty() {
            Typing::Unresolved
        } else {
            // TODO(validation): all expressions in the array should have the
            // same (or implicitly convertible) types
            if let Some(element_type) = self
                .typing_of_expression(node.items.first().unwrap())
                .as_type_id()
            {
                let type_id = self.types.register_type(Type::Array {
                    element_type,
                    location: DataLocation::Memory,
                });
                Typing::Resolved(type_id)
            } else {
                Typing::Unresolved
            }
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let typing = match &node.arguments {
            input_ir::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                positional_arguments,
            ) => self.typing_of_function_call_with_positional_arguments(
                node,
                &positional_arguments.arguments,
            ),
            input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(named_arguments) => {
                if let Some(named_arguments) = &named_arguments.arguments {
                    self.typing_of_function_call_with_named_arguments(
                        node,
                        &named_arguments.arguments,
                    )
                } else {
                    self.typing_of_function_call_with_named_arguments(node, &[])
                }
            }
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn enter_call_options_expression(&mut self, node: &input_ir::CallOptionsExpression) -> bool {
        for option in &node.options {
            let identifier = &option.name;
            let resolution = self
                .built_ins_resolver()
                .lookup_call_option(identifier.unparse().as_str())
                .into();
            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);
        }
        true
    }

    fn leave_call_options_expression(&mut self, node: &input_ir::CallOptionsExpression) {
        let typing = self.typing_of_expression(&node.operand);
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn enter_yul_path(&mut self, items: &input_ir::YulPath) -> bool {
        if items.is_empty() {
            return false;
        }

        let scope_id = self.current_scope_id();
        let consumed_identifiers = if self.language_version >= VERSION_0_7_0 {
            let identifier = &items[0];
            let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier.unparse());
            let reference = Reference::new(Rc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            if items.len() > 1 {
                let suffix = &items[1];
                let resolution = self.resolve_yul_suffix(&suffix.unparse(), &resolution);
                let reference = Reference::new(Rc::clone(suffix), resolution);
                self.binder.insert_reference(reference);
            }

            2
        } else {
            let identifier = &items[0];
            let identifier_text = identifier.unparse();
            let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier_text);
            let resolution = if resolution == Resolution::Unresolved {
                // if the identifier cannot be resolved as-is, try to extract a
                // suffix (ie. `_slot`, `_offset`, etc.) and resolve for it
                if let Some((prefix, suffix)) = identifier_text.rsplit_once('_') {
                    let resolution = self.resolve_symbol_in_yul_scope(scope_id, prefix);
                    self.resolve_yul_suffix(suffix, &resolution)
                } else {
                    resolution
                }
            } else {
                resolution
            };

            let reference = Reference::new(Rc::clone(identifier), resolution);
            self.binder.insert_reference(reference);

            1
        };

        // any remaining identifiers cannot be resolved, but we still want to
        // emit a reference for each of them
        for identifier in items.iter().skip(consumed_identifiers) {
            self.binder.insert_reference(Reference::new(
                Rc::clone(identifier),
                Resolution::Unresolved,
            ));
        }

        false
    }

    fn enter_yul_stack_assignment_statement(
        &mut self,
        node: &input_ir::YulStackAssignmentStatement,
    ) -> bool {
        let identifier = &node.variable;
        let scope_id = self.current_scope_id();
        let resolution = self.resolve_symbol_in_yul_scope(scope_id, &identifier.unparse());
        let reference = Reference::new(Rc::clone(identifier), resolution);
        self.binder.insert_reference(reference);

        false
    }

    fn enter_yul_for_statement(&mut self, node: &input_ir::YulForStatement) -> bool {
        // Visit the initialization block first
        input_ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block. This is mainly so references in the condition
        // can resolve against the initialization block. The iterator and body
        // should be already properly linked from construction.
        self.enter_scope_for_node_id(node.initialization.node_id);
        input_ir::visitor::accept_yul_expression(&node.condition, self);
        input_ir::visitor::accept_yul_block(&node.iterator, self);
        input_ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.node_id);

        // We already visited our children
        false
    }

    fn enter_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) -> bool {
        if node.var_keyword.is_some() {
            // this is a (deprecated) variable declaration, not assignment
            return true;
        }

        for element in &node.elements {
            let Some(member) = &element.member else {
                continue;
            };
            match member {
                input_ir::TupleMember::TypedTupleMember(_) => {
                    // this is a declaration, not a reference, so nothing left to do in this pass
                }
                input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) => {
                    let identifier = &untyped_tuple_member.name;
                    let scope_id = self.current_scope_id();
                    let resolution = self.resolve_symbol_in_scope(scope_id, &identifier.unparse());
                    let reference = Reference::new(Rc::clone(identifier), resolution);
                    self.binder.insert_reference(reference);
                }
            }
        }

        true
    }

    fn leave_emit_statement(&mut self, node: &input_ir::EmitStatement) {
        if let input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(
            named_arguments_declaration,
        ) = &node.arguments
        {
            if let Some(named_arguments) = &named_arguments_declaration.arguments {
                let definition_id = self
                    .binder
                    .find_reference_by_identifier_node_id(node.event.last().unwrap().id())
                    .and_then(|reference| reference.resolution.as_definition_id());
                self.resolve_named_arguments(&named_arguments.arguments, definition_id);
            }
        }
    }

    fn leave_revert_statement(&mut self, node: &input_ir::RevertStatement) {
        if let input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(
            named_arguments_declaration,
        ) = &node.arguments
        {
            if let Some(named_arguments) = &named_arguments_declaration.arguments {
                let definition_id = node
                    .error
                    .as_ref()
                    .and_then(|error| {
                        self.binder
                            .find_reference_by_identifier_node_id(error.last().unwrap().id())
                    })
                    .and_then(|reference| reference.resolution.as_definition_id());
                self.resolve_named_arguments(&named_arguments.arguments, definition_id);
            }
        }
    }

    fn leave_variable_declaration_statement(
        &mut self,
        node: &input_ir::VariableDeclarationStatement,
    ) {
        if matches!(
            node.variable_type,
            input_ir::VariableDeclarationType::VarKeyword
        ) {
            // update the type of the variable with the type of the expression (if available)
            if let Some(value) = &node.value {
                let typing = self.typing_of_expression(&value.expression);
                self.binder.fixup_node_typing(node.node_id, typing);
            }
        }
    }

    fn leave_tuple_deconstruction_statement(
        &mut self,
        node: &input_ir::TupleDeconstructionStatement,
    ) {
        if node.var_keyword.is_none() {
            return;
        }

        let typing = self.typing_of_expression(&node.expression);
        let Typing::Resolved(tuple_type_id) = typing else {
            // we can't fixup typing if the expression failed to type
            return;
        };
        let types = if let Type::Tuple { types } = self.types.get_type_by_id(tuple_type_id) {
            types.as_slice()
        } else {
            // the resolved type is not a tuple
            &[tuple_type_id]
        };

        // fixup typing of `var` declarations
        for (element, element_type_id) in node.elements.iter().zip(types) {
            let Some(member) = &element.member else {
                continue;
            };
            if let input_ir::TupleMember::UntypedTupleMember(untyped_tuple_member) = member {
                let typing = Typing::Resolved(*element_type_id);
                self.binder
                    .fixup_node_typing(untyped_tuple_member.node_id, typing);
            }
        }
    }
}
