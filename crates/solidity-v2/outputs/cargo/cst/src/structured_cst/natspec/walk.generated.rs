// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! Attaches the `NatSpec` comments to the CST nodes they document, only walking the nodes that are
//! documentable, or can contain one.

use crate::structured_cst::natspec::NatSpec;
use crate::structured_cst::nodes;
use crate::structured_cst::text_range::TextStart;

/// Attaches to each documentable node of `source_unit` the `NatSpec` comment that `natspec_at`
/// returns for the offset where the node starts.
pub fn attach_natspec(
    source_unit: &mut nodes::SourceUnit,
    natspec_at: &impl Fn(usize) -> Option<NatSpec>,
) {
    attach_source_unit(source_unit, natspec_at);
}

fn attach_assembly_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::AssemblyStatement,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_block<F: Fn(usize) -> Option<NatSpec>>(node: &mut nodes::Block, natspec_at: &F) {
    attach_statements(&mut node.statements, natspec_at);
}

fn attach_catch_clause<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::CatchClause,
    natspec_at: &F,
) {
    attach_block(&mut node.body, natspec_at);
}

fn attach_constant_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ConstantDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_constructor_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ConstructorDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_block(&mut node.body, natspec_at);
}

fn attach_contract_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ContractDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_contract_members(&mut node.members, natspec_at);
}

fn attach_do_while_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::DoWhileStatement,
    natspec_at: &F,
) {
    attach_statement(&mut node.body, natspec_at);
}

fn attach_else_branch<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ElseBranch,
    natspec_at: &F,
) {
    attach_statement(&mut node.body, natspec_at);
}

fn attach_enum_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::EnumDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_error_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ErrorDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_event_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::EventDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_fallback_function_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::FallbackFunctionDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_function_body(&mut node.body, natspec_at);
}

fn attach_for_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ForStatement,
    natspec_at: &F,
) {
    attach_statement(&mut node.body, natspec_at);
}

fn attach_function_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::FunctionDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_function_body(&mut node.body, natspec_at);
}

fn attach_if_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::IfStatement,
    natspec_at: &F,
) {
    attach_statement(&mut node.body, natspec_at);
    if let Some(child) = &mut node.else_branch {
        attach_else_branch(child, natspec_at);
    }
}

fn attach_interface_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::InterfaceDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_interface_members(&mut node.members, natspec_at);
}

fn attach_library_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::LibraryDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_library_members(&mut node.members, natspec_at);
}

fn attach_modifier_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ModifierDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_function_body(&mut node.body, natspec_at);
}

fn attach_receive_function_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ReceiveFunctionDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
    attach_function_body(&mut node.body, natspec_at);
}

fn attach_source_unit<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::SourceUnit,
    natspec_at: &F,
) {
    attach_source_unit_members(&mut node.members, natspec_at);
}

fn attach_state_variable_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::StateVariableDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_struct_definition<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::StructDefinition,
    natspec_at: &F,
) {
    if let Some(start) = node.calculate_text_start() {
        node.natspec = natspec_at(start);
    }
}

fn attach_try_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::TryStatement,
    natspec_at: &F,
) {
    attach_block(&mut node.body, natspec_at);
    attach_catch_clauses(&mut node.catch_clauses, natspec_at);
}

fn attach_unchecked_block<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::UncheckedBlock,
    natspec_at: &F,
) {
    attach_block(&mut node.block, natspec_at);
}

fn attach_while_statement<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::WhileStatement,
    natspec_at: &F,
) {
    attach_statement(&mut node.body, natspec_at);
}

fn attach_contract_member<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ContractMember,
    natspec_at: &F,
) {
    // Only the variants that can contain a documentable node are walked: the `_` arm may be
    // unreachable, or the only other arm
    #[allow(
        unreachable_patterns,
        clippy::match_wildcard_for_single_variants,
        clippy::single_match
    )]
    match node {
        nodes::ContractMember::FunctionDefinition(child) => {
            attach_function_definition(child, natspec_at);
        }
        nodes::ContractMember::ConstructorDefinition(child) => {
            attach_constructor_definition(child, natspec_at);
        }
        nodes::ContractMember::ReceiveFunctionDefinition(child) => {
            attach_receive_function_definition(child, natspec_at);
        }
        nodes::ContractMember::FallbackFunctionDefinition(child) => {
            attach_fallback_function_definition(child, natspec_at);
        }
        nodes::ContractMember::ModifierDefinition(child) => {
            attach_modifier_definition(child, natspec_at);
        }
        nodes::ContractMember::StructDefinition(child) => {
            attach_struct_definition(child, natspec_at);
        }
        nodes::ContractMember::EnumDefinition(child) => {
            attach_enum_definition(child, natspec_at);
        }
        nodes::ContractMember::EventDefinition(child) => {
            attach_event_definition(child, natspec_at);
        }
        nodes::ContractMember::ErrorDefinition(child) => {
            attach_error_definition(child, natspec_at);
        }
        nodes::ContractMember::StateVariableDefinition(child) => {
            attach_state_variable_definition(child, natspec_at);
        }
        _ => {}
    }
}

fn attach_function_body<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::FunctionBody,
    natspec_at: &F,
) {
    // Only the variants that can contain a documentable node are walked: the `_` arm may be
    // unreachable, or the only other arm
    #[allow(
        unreachable_patterns,
        clippy::match_wildcard_for_single_variants,
        clippy::single_match
    )]
    match node {
        nodes::FunctionBody::Block(child) => {
            attach_block(child, natspec_at);
        }
        _ => {}
    }
}

fn attach_source_unit_member<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::SourceUnitMember,
    natspec_at: &F,
) {
    // Only the variants that can contain a documentable node are walked: the `_` arm may be
    // unreachable, or the only other arm
    #[allow(
        unreachable_patterns,
        clippy::match_wildcard_for_single_variants,
        clippy::single_match
    )]
    match node {
        nodes::SourceUnitMember::ContractDefinition(child) => {
            attach_contract_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::InterfaceDefinition(child) => {
            attach_interface_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::LibraryDefinition(child) => {
            attach_library_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::StructDefinition(child) => {
            attach_struct_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::EnumDefinition(child) => {
            attach_enum_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::FunctionDefinition(child) => {
            attach_function_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::ErrorDefinition(child) => {
            attach_error_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::EventDefinition(child) => {
            attach_event_definition(child, natspec_at);
        }
        nodes::SourceUnitMember::ConstantDefinition(child) => {
            attach_constant_definition(child, natspec_at);
        }
        _ => {}
    }
}

fn attach_statement<F: Fn(usize) -> Option<NatSpec>>(node: &mut nodes::Statement, natspec_at: &F) {
    // Only the variants that can contain a documentable node are walked: the `_` arm may be
    // unreachable, or the only other arm
    #[allow(
        unreachable_patterns,
        clippy::match_wildcard_for_single_variants,
        clippy::single_match
    )]
    match node {
        nodes::Statement::IfStatement(child) => {
            attach_if_statement(child, natspec_at);
        }
        nodes::Statement::ForStatement(child) => {
            attach_for_statement(child, natspec_at);
        }
        nodes::Statement::WhileStatement(child) => {
            attach_while_statement(child, natspec_at);
        }
        nodes::Statement::DoWhileStatement(child) => {
            attach_do_while_statement(child, natspec_at);
        }
        nodes::Statement::TryStatement(child) => {
            attach_try_statement(child, natspec_at);
        }
        nodes::Statement::AssemblyStatement(child) => {
            attach_assembly_statement(child, natspec_at);
        }
        nodes::Statement::Block(child) => {
            attach_block(child, natspec_at);
        }
        nodes::Statement::UncheckedBlock(child) => {
            attach_unchecked_block(child, natspec_at);
        }
        _ => {}
    }
}

fn attach_catch_clauses<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::CatchClauses,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_catch_clause(element, natspec_at);
    }
}

fn attach_contract_members<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::ContractMembers,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_contract_member(element, natspec_at);
    }
}

fn attach_interface_members<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::InterfaceMembers,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_contract_member(element, natspec_at);
    }
}

fn attach_library_members<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::LibraryMembers,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_contract_member(element, natspec_at);
    }
}

fn attach_source_unit_members<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::SourceUnitMembers,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_source_unit_member(element, natspec_at);
    }
}

fn attach_statements<F: Fn(usize) -> Option<NatSpec>>(
    node: &mut nodes::Statements,
    natspec_at: &F,
) {
    for element in &mut node.elements {
        attach_statement(element, natspec_at);
    }
}
