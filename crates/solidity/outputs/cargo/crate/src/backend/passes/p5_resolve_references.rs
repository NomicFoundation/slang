use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p4_linearise_contracts::Output as Input;
use crate::backend::binder::{
    Binder, Definition, Reference, Resolution, Scope, ScopeId, Typing, UsingDirective,
};
use crate::backend::built_ins::BuiltInsResolver;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::{DataLocation, Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

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
        // TODO: we need to do hierarchy lookups for contracts and interfaces if
        // we're in the scope of a contract/interface/library. This we probably
        // cannot delegate to the binder.
        let resolution = self.binder.resolve_in_scope(scope_id, symbol);
        if resolution == Resolution::Unresolved {
            self.built_ins_resolver().lookup_global(symbol).into()
        } else {
            resolution
        }
    }

    fn active_using_directives_for_type(&self, type_id: TypeId) -> Vec<&UsingDirective> {
        // Compute the canonical type: this handles cases where the given type
        // is context dependent:
        // - If the type is a reference type, we need to compute the id of the
        // location independent type (using DataLocation::Inherited). If it
        // doesn't exist, proceed with the given value, but we won't find any
        // type-specific directives, only those applicable to all types (ie.
        // `using L for *`)
        // - If the type is a function type, it may have an associated
        // definition ID from the function definition where it is derived from.
        let canonical_type = self.types.get_type_by_id(type_id).canonicalize();
        let type_id = self.types.find_type(&canonical_type).unwrap_or(type_id);

        let mut directives = Vec::new();
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            let scope_directives = scope.get_using_directives();
            directives.extend(
                scope_directives
                    .iter()
                    .filter(|directive| directive.applies_to(type_id)),
            );
        }
        let global_directives = self.binder.get_global_using_directives();
        directives.extend(
            global_directives
                .iter()
                .filter(|directive| directive.applies_to(type_id)),
        );
        directives
    }

    fn resolve_symbol_in_typing(&self, typing: &Typing, symbol: &str) -> Resolution {
        match typing {
            Typing::Unresolved | Typing::Undetermined(_) => Resolution::Unresolved,
            Typing::Resolved(type_id) => {
                let resolution = self.resolve_symbol_in_type(*type_id, symbol);
                if resolution == Resolution::Unresolved {
                    // Consider active `using` directives in the current context
                    let active_directives = self.active_using_directives_for_type(*type_id);
                    let mut definition_ids = Vec::new();
                    for directive in &active_directives {
                        let scope_id = directive.get_scope_id();
                        let ids = self
                            .binder
                            .resolve_in_scope_as_namespace(scope_id, symbol)
                            .get_definition_ids();
                        // TODO: filter the resolved definitions to only include
                        // functions whose first parameter is of our type (or
                        // implicitly convertible to it)
                        definition_ids.extend(ids);
                    }
                    Resolution::from(definition_ids)
                } else {
                    resolution
                }
            }
            Typing::This | Typing::Super => {
                // TODO: restrict lookup to the scope of the enclosing contract,
                // but extend the search to the bases contracts/interfaces using
                // the linearisation information
                Resolution::Unresolved
            }
            Typing::New(_type_id) => {
                // TODO: resolve legacy constructor call options (ie. `(new Lock).value(1)()`)
                Resolution::Unresolved
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
            Type::Contract { .. } => {
                // TODO: expose the public contract methods
                Resolution::Unresolved
            }
            Type::Interface { .. } => {
                // TODO: expose the public interface methods
                Resolution::Unresolved
            }
            Type::Struct { definition_id, .. } => {
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
            }
            _ => self
                .built_ins_resolver()
                .lookup_member_of_type(type_, symbol)
                .into(),
        }
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
            input_ir::Expression::HexNumberExpression(_)
            | input_ir::Expression::DecimalNumberExpression(_) => {
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

    fn types_of_tuple(&self, items: &[input_ir::TupleValue]) -> Option<Vec<TypeId>> {
        let mut result = Vec::new();
        for item in items {
            let expression = item.expression.as_ref()?;
            let type_id = self.typing_of_expression(expression).as_type_id()?;
            result.push(type_id);
        }
        Some(result)
    }

    fn typing_of_resolution(&self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Unresolved => Typing::Unresolved,
            Resolution::BuiltIn(built_in) => self.built_ins_resolver().typing_of(built_in),
            Resolution::Definition(definition_id) => self.binder.node_typing(*definition_id),
            Resolution::Ambiguous(definitions) => {
                // TODO: we need some sort of link between the definition_id and the resulting type
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

    fn current_contract_scope_id(&self) -> ScopeId {
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            if matches!(scope, Scope::Contract(_)) {
                return *scope_id;
            }
        }
        unreachable!("no contract scope found in the stack");
    }

    fn resolve_modifier_invocation(&mut self, modifier_invocation: &input_ir::ModifierInvocation) {
        let identifier_path = &modifier_invocation.name;
        let mut scope_id = Some(self.current_contract_scope_id());
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
        named_arguments_declaration: &input_ir::NamedArgumentsDeclaration,
        definition_id: Option<NodeId>,
    ) {
        let Some(named_arguments) = named_arguments_declaration
            .arguments
            .as_ref()
            .map(|arguments| &arguments.arguments)
        else {
            return;
        };
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
        // TODO: we need to do hierarchy lookups for contracts and interfaces if
        // we're in the scope of a contract/interface/library. This we probably
        // cannot delegate to the binder.
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
        let type_id = self.typing_of_expression(&node.left_operand).as_type_id();
        // TODO(validation): check that the left operand is an integer and the
        // right operand is an _unsigned_ integer
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
        let types = self.types_of_tuple(&node.items);
        let type_id = types.map(|types| {
            if types.len() == 1 {
                types.first().copied().unwrap()
            } else {
                self.types.register_type(Type::Tuple { types })
            }
        });
        self.binder.set_node_type(node.node_id, type_id);
    }

    fn leave_member_access_expression(&mut self, node: &input_ir::MemberAccessExpression) {
        // we need to resolve the identifier at this point that we already have
        // typing information of the operand expression
        let operand_typing = self.typing_of_expression(&node.operand);
        let resolution = self.resolve_symbol_in_typing(&operand_typing, &node.member.unparse());

        let typing = self.typing_of_resolution(&resolution);
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
            self.typing_of_expression(node.items.first().unwrap())
        };
        self.binder.set_node_typing(node.node_id, typing);
    }

    fn leave_function_call_expression(&mut self, node: &input_ir::FunctionCallExpression) {
        let operand_typing = self.typing_of_expression(&node.operand);
        let positional_argument_typings =
            if let input_ir::ArgumentsDeclaration::PositionalArgumentsDeclaration(arguments) =
                &node.arguments
            {
                Some(
                    arguments
                        .arguments
                        .iter()
                        .map(|argument| self.typing_of_expression(argument))
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };

        let (typing, definition_id) = match operand_typing {
            Typing::Unresolved | Typing::This | Typing::Super => {
                // `this` and `super` are not callable
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => {
                let operand_type = self.types.get_type_by_id(type_id);
                match operand_type {
                    Type::Function {
                        definition_id,
                        return_type,
                        ..
                    } => (Typing::Resolved(*return_type), *definition_id),
                    // TODO: support other types that can be called
                    _ => {
                        // TODO(validation): the operand did not resolve to a function
                        (Typing::Unresolved, None)
                    }
                }
            }
            Typing::Undetermined(_type_ids) => {
                // TODO: resolve argument types and match best overload
                // TODO: maybe update the typing of the operand?
                // TODO: return the definition_id used to later resolve named arguments
                (Typing::Unresolved, None)
            }
            Typing::MetaType(type_) => {
                // TODO(validation): this is a cast to the given type, but we
                // need to verify that the (single) argument is convertible
                if let Some(type_id_of_first_argument) = positional_argument_typings
                    .and_then(|arguments| arguments.first().cloned())
                    .and_then(|typing| typing.as_type_id())
                {
                    let data_location = self
                        .types
                        .get_type_by_id(type_id_of_first_argument)
                        .data_location();
                    let type_id = self
                        .types
                        .register_type(type_.with_data_location(data_location));
                    (Typing::Resolved(type_id), None)
                } else {
                    (Typing::Unresolved, None)
                }
            }
            Typing::New(type_id) => {
                // TODO(validation): check that the given type is actually constructible
                let type_ = self.types.get_type_by_id(type_id);
                let definition_id = match type_ {
                    Type::Contract { definition_id } => Some(*definition_id),
                    Type::Struct { definition_id, .. } => Some(*definition_id),
                    _ => None,
                };
                (Typing::Resolved(type_id), definition_id)
            }
            Typing::UserMetaType(node_id) => {
                // Generally this is a cast to the underlying type of the given
                // definition, except for structs for which we need to construct
                // the value in memory
                match self.binder.find_definition_by_id(node_id) {
                    Some(Definition::Contract(_) | Definition::Interface(_)) => {
                        // TODO(validation): the type of the first argument should be an address
                        let type_id = self.types.register_type(Type::Contract {
                            definition_id: node_id,
                        });
                        (Typing::Resolved(type_id), None)
                    }
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
            Typing::BuiltIn(built_in) => {
                let typing = if let Some(positional_argument_typings) = positional_argument_typings
                {
                    self.built_ins_resolver()
                        .typing_of_function_call(&built_in, &positional_argument_typings)
                } else {
                    // built-ins cannot be called with named arguments
                    Typing::Unresolved
                };
                (typing, None)
            }
        };
        self.binder.set_node_typing(node.node_id, typing);

        // Reference and resolve named arguments
        if let input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(
            named_arguments_declaration,
        ) = &node.arguments
        {
            self.resolve_named_arguments(named_arguments_declaration, definition_id);
        }
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
            let definition_id = self
                .binder
                .find_reference_by_identifier_node_id(node.event.last().unwrap().id())
                .and_then(|reference| reference.resolution.as_definition_id());
            self.resolve_named_arguments(named_arguments_declaration, definition_id);
        }
    }

    fn leave_revert_statement(&mut self, node: &input_ir::RevertStatement) {
        if let input_ir::ArgumentsDeclaration::NamedArgumentsDeclaration(
            named_arguments_declaration,
        ) = &node.arguments
        {
            let definition_id = node
                .error
                .as_ref()
                .and_then(|error| {
                    self.binder
                        .find_reference_by_identifier_node_id(error.last().unwrap().id())
                })
                .and_then(|reference| reference.resolution.as_definition_id());
            self.resolve_named_arguments(named_arguments_declaration, definition_id);
        }
    }
}
