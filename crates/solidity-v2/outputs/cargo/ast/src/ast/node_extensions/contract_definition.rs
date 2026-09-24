use std::sync::Arc;

use slang_solidity_v2_semantic::context::VirtualTarget as SemanticVirtualTarget;

use super::super::nodes::{
    create_error_definition, create_event_definition, create_function_definition,
    create_state_variable_definition,
};
use super::super::{
    ContractDefinition, ContractDefinitionStruct, Definition, ErrorDefinition, EventDefinition,
    FunctionDefinition, FunctionKind, FunctionMutability, ModifierInvocation,
    StateVariableDefinition,
};
use super::{ContractBase, VirtualTarget};

impl ContractDefinitionStruct {
    pub fn direct_bases(&self) -> Vec<ContractBase> {
        self.inheritance_types()
            .iter()
            .filter_map(|inheritance_type| {
                let base = inheritance_type.type_name().resolve_to_definition()?;
                ContractBase::from_definition(&base)
            })
            .collect()
    }

    /// Returns the list of contracts/interfaces in the hierarchy (including
    /// self) in the order given by the C3 linearisation, with self contract
    /// always first
    pub fn linearised_bases(&self) -> Vec<ContractBase> {
        let Some(base_node_ids) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.id())
        else {
            // TODO(validation) SDR[4]: once we have validation implemented, this
            // branch should not be reachable, or we should generate an error
            // while building the `SemanticAnalysis`.
            return Vec::new();
        };
        base_node_ids
            .iter()
            .map(|node_id| {
                let base_definition =
                    Definition::try_create(*node_id, &self.semantic).expect("node is a definition");
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
            })
            .collect()
    }

    pub fn state_variables(&self) -> Vec<StateVariableDefinition> {
        self.members().iter_state_variable_definitions().collect()
    }

    /// Returns the list of state variable definitions in the order laid out in storage
    pub fn linearised_state_variables(&self) -> Vec<StateVariableDefinition> {
        self.semantic
            .linearised_state_variables(self.ir_node.id())
            .iter()
            .map(|ir_node| create_state_variable_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| {
                matches!(
                    function.kind(),
                    FunctionKind::Regular | FunctionKind::Fallback | FunctionKind::Receive
                )
            })
            .collect()
    }

    /// Whether the contract accepts a plain ETH transfer: it, or any base in
    /// its linearisation, declares a `receive()` function or a `payable`
    /// `fallback()` (solc's `ContractType::isPayable`).
    pub fn is_payable(&self) -> bool {
        self.linearised_functions().iter().any(|function| {
            matches!(function.kind(), FunctionKind::Receive)
                || (matches!(function.kind(), FunctionKind::Fallback)
                    && matches!(
                        function.attributes().mutability(),
                        FunctionMutability::Payable
                    ))
        })
    }

    pub fn modifiers(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| matches!(function.kind(), FunctionKind::Modifier))
            .collect()
    }

    pub fn constructor(&self) -> Option<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .find(|function| matches!(function.kind(), FunctionKind::Constructor))
    }

    /// Returns the list of functions defined in all the hierarchy of the
    /// contract, in alphabetical order
    pub fn linearised_functions(&self) -> Vec<FunctionDefinition> {
        self.semantic
            .linearised_functions(self.ir_node.id())
            .iter()
            .map(|ir_node| create_function_definition(ir_node, &self.semantic))
            .collect()
    }

    /// Resolves `function` in code compiled into this contract. Virtual members
    /// select their most-derived override, including public variable getters;
    /// nonvirtual members and declarations with no override resolve to themselves.
    ///
    /// Returns `None` unless `function` is a function or modifier declared in
    /// this contract's hierarchy and belongs to the same compilation unit.
    /// Constructors and free or library functions are not accepted.
    pub fn resolve_virtual(&self, function: &FunctionDefinition) -> Option<VirtualTarget> {
        Some(
            match self
                .semantic
                .resolve_virtual(self.ir_node.id(), &function.ir_node)?
            {
                SemanticVirtualTarget::Function(target) => {
                    VirtualTarget::Function(create_function_definition(target, &self.semantic))
                }
                SemanticVirtualTarget::Getter(state_variable) => VirtualTarget::Getter(
                    create_state_variable_definition(state_variable, &self.semantic),
                ),
            },
        )
    }

    /// Resolves `super.f` for `function`, written in `enclosing_contract` and
    /// compiled into this contract. Returns the nearest implemented regular
    /// function after the enclosing contract in this contract's linearisation.
    ///
    /// Returns `None` if either argument is outside this contract's hierarchy
    /// or compilation unit, `function` is not regular, or no implementation
    /// follows. Super targets are functions, never getters.
    pub fn resolve_super(
        &self,
        function: &FunctionDefinition,
        enclosing_contract: &ContractDefinition,
    ) -> Option<FunctionDefinition> {
        if !Arc::ptr_eq(&self.semantic, &enclosing_contract.semantic) {
            return None;
        }
        Some(create_function_definition(
            self.semantic.resolve_super(
                self.ir_node.id(),
                &function.ir_node,
                enclosing_contract.node_id(),
            )?,
            &self.semantic,
        ))
    }

    /// Finds the modifier that `invocation` runs in code compiled into this
    /// contract. A bare `m` runs the most-derived override of `m`. A qualified
    /// `A.m` always runs `A`'s own `m`, and so does a `m` that is not virtual.
    ///
    /// Returns `None` if `invocation` comes from another compilation unit, if
    /// it calls a base constructor instead of a modifier, or if the modifier
    /// is not declared in this contract or its bases.
    pub fn resolve_modifier(&self, invocation: &ModifierInvocation) -> Option<FunctionDefinition> {
        Some(create_function_definition(
            self.semantic
                .resolve_modifier(self.ir_node.id(), &invocation.ir_node)?,
            &self.semantic,
        ))
    }

    pub fn errors(&self) -> Vec<ErrorDefinition> {
        self.members().iter_error_definitions().collect()
    }

    pub fn linearised_errors(&self) -> Vec<ErrorDefinition> {
        self.semantic
            .linearised_errors(self.ir_node.id())
            .iter()
            .map(|ir_node| create_error_definition(ir_node, &self.semantic))
            .collect()
    }

    pub fn events(&self) -> Vec<EventDefinition> {
        self.members().iter_event_definitions().collect()
    }

    pub fn linearised_events(&self) -> Vec<EventDefinition> {
        self.semantic
            .linearised_events(self.ir_node.id())
            .iter()
            .map(|ir_node| create_event_definition(ir_node, &self.semantic))
            .collect()
    }
}
