use std::collections::HashMap;
use std::rc::Rc;

use super::p0_build_ast::Output as Input;
use crate::backend::ir::ir2_flat_contracts::transformer::Transformer;
use crate::backend::ir::ir2_flat_contracts::{self as output, input, SourceUnit};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

/// This pass is reserved to make ergonomic changes to the AST in order to make
/// it easier to use. For now, it will only flatten contract specifiers:
/// inheritance and storage layout specifiers. In the future, more
/// transformations will be added.
pub fn run(input: Input) -> Output {
    let mut pass = Pass {};
    let files = input
        .files
        .iter()
        .map(|(file_id, source_unit)| (file_id.clone(), pass.transform_source_unit(source_unit)))
        .collect();
    let compilation_unit = input.compilation_unit;
    Output {
        compilation_unit,
        files,
    }
}

struct Pass {}

impl Transformer for Pass {
    fn transform_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition {
        let node_id = source.node_id;
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
        let name = Rc::clone(&source.name);
        let members = self.transform_contract_members(&source.members);
        let inheritance_types = source
            .specifiers
            .iter()
            .find_map(|specifier| {
                if let input::ContractSpecifier::InheritanceSpecifier(inheritance) = specifier {
                    Some(self.transform_inheritance_types(&inheritance.types))
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let storage_layout = source.specifiers.iter().find_map(|specifier| {
            if let input::ContractSpecifier::StorageLayoutSpecifier(storage_layout) = specifier {
                Some(self.transform_storage_layout_specifier(storage_layout))
            } else {
                None
            }
        });

        Rc::new(output::ContractDefinitionStruct {
            node_id,
            abstract_keyword,
            name,
            members,
            inheritance_types,
            storage_layout,
        })
    }

    fn transform_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition {
        let node_id = source.node_id;
        let name = self.transform_function_name(&source.name);
        let attributes = self.transform_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_parameters(&returns.variables.parameters));
        Rc::new(output::FunctionDefinitionStruct {
            node_id,
            name,
            attributes,
            body,
            parameters,
            returns,
        })
    }

    fn transform_constructor_definition(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::ConstructorDefinition {
        let node_id = source.node_id;
        let attributes = self.transform_constructor_attributes(&source.attributes);
        let body = self.transform_block(&source.body);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        Rc::new(output::ConstructorDefinitionStruct {
            node_id,
            attributes,
            body,
            parameters,
        })
    }

    fn transform_unnamed_function_definition(
        &mut self,
        source: &input::UnnamedFunctionDefinition,
    ) -> output::UnnamedFunctionDefinition {
        let node_id = source.node_id;
        let attributes = self.transform_unnamed_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        Rc::new(output::UnnamedFunctionDefinitionStruct {
            node_id,
            attributes,
            body,
            parameters,
        })
    }

    fn transform_fallback_function_definition(
        &mut self,
        source: &input::FallbackFunctionDefinition,
    ) -> output::FallbackFunctionDefinition {
        let node_id = source.node_id;
        let attributes = self.transform_fallback_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_parameters(&returns.variables.parameters));
        Rc::new(output::FallbackFunctionDefinitionStruct {
            node_id,
            attributes,
            body,
            parameters,
            returns,
        })
    }

    fn transform_receive_function_definition(
        &mut self,
        source: &input::ReceiveFunctionDefinition,
    ) -> output::ReceiveFunctionDefinition {
        let node_id = source.node_id;
        let attributes = self.transform_receive_function_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        Rc::new(output::ReceiveFunctionDefinitionStruct {
            node_id,
            attributes,
            body,
            parameters,
        })
    }

    fn transform_modifier_definition(
        &mut self,
        source: &input::ModifierDefinition,
    ) -> output::ModifierDefinition {
        let node_id = source.node_id;
        let name = Rc::clone(&source.name);
        let attributes = self.transform_modifier_attributes(&source.attributes);
        let body = self.transform_function_body(&source.body);
        let parameters = source
            .parameters
            .as_ref()
            .map(|parameters| self.transform_parameters(&parameters.parameters));
        Rc::new(output::ModifierDefinitionStruct {
            node_id,
            name,
            attributes,
            body,
            parameters,
        })
    }

    fn transform_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let node_id = source.node_id;
        let attributes = self.transform_function_type_attributes(&source.attributes);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_parameters(&returns.variables.parameters));
        Rc::new(output::FunctionTypeStruct {
            node_id,
            attributes,
            parameters,
            returns,
        })
    }

    fn transform_try_statement(&mut self, source: &input::TryStatement) -> output::TryStatement {
        let node_id = source.node_id;
        let expression = self.transform_expression(&source.expression);
        let body = self.transform_block(&source.body);
        let catch_clauses = self.transform_catch_clauses(&source.catch_clauses);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_parameters(&returns.variables.parameters));
        Rc::new(output::TryStatementStruct {
            node_id,
            expression,
            body,
            catch_clauses,
            returns,
        })
    }

    fn transform_catch_clause_error(
        &mut self,
        source: &input::CatchClauseError,
    ) -> output::CatchClauseError {
        let node_id = source.node_id;
        let name = source.name.as_ref().map(Rc::clone);
        let parameters = self.transform_parameters(&source.parameters.parameters);
        Rc::new(output::CatchClauseErrorStruct {
            node_id,
            name,
            parameters,
        })
    }

    fn transform_yul_function_definition(
        &mut self,
        source: &input::YulFunctionDefinition,
    ) -> output::YulFunctionDefinition {
        let node_id = source.node_id;
        let name = Rc::clone(&source.name);
        let body = self.transform_yul_block(&source.body);
        let parameters = self.transform_yul_parameters(&source.parameters.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|returns| self.transform_yul_variable_names(&returns.variables));
        Rc::new(output::YulFunctionDefinitionStruct {
            node_id,
            name,
            body,
            parameters,
            returns,
        })
    }
}
