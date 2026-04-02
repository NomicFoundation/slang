use std::cmp::Ordering;
use std::ops::Div;
use std::rc::Rc;

use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::super::{
    ContractDefinition, ContractDefinitionStruct, ContractMember, ContractMembersStruct,
    Definition, ErrorDefinition, EventDefinition, FunctionDefinition, FunctionKind,
    InterfaceDefinition, InterfaceMembersStruct, StateVariableDefinition,
};
use crate::abi::{AbiEntry, ContractAbi, StorageItem};
use crate::ast::StateVariableMutability;

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    fn from_definition(definition: &Definition) -> Option<Self> {
        match definition {
            Definition::Contract(contract) => Some(Self::Contract(Rc::clone(contract))),
            Definition::Interface(interface) => Some(Self::Interface(Rc::clone(interface))),
            _ => None,
        }
    }
}

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
    pub fn compute_linearised_bases(&self) -> Vec<ContractBase> {
        let Some(base_node_ids) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.id())
        else {
            // TODO(validation): once we have validation implemented, this
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
    pub fn compute_linearised_state_variables(&self) -> Vec<StateVariableDefinition> {
        let mut state_variables = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            let ContractBase::Contract(contract) = base else {
                continue;
            };
            state_variables.extend(contract.members().iter_state_variable_definitions());
        }
        state_variables
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
    pub fn compute_linearised_functions(&self) -> Vec<FunctionDefinition> {
        let mut functions: Vec<FunctionDefinition> = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in &bases {
            // TODO(validation): we don't pick up functions defined in
            // interfaces because they should be implemented in inheriting
            // contracts, but this is not yet enforced anywhere
            let ContractBase::Contract(contract) = base else {
                continue;
            };

            // Handle function overriding
            let contract_functions = contract
                .functions()
                .into_iter()
                .filter(|function| {
                    // check the existing functions and remove any duplicates
                    // because they should be overridden by them
                    let existing = functions
                        .iter()
                        .any(|linearised_function| linearised_function.overrides(function));
                    !existing
                })
                // collect to avoid holding the read-borrow on `functions`
                .collect::<Vec<_>>();

            functions.extend(contract_functions);
        }

        // sort returned functions by name
        functions.sort_by(|a, b| match (a.name(), b.name()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.name().cmp(&b.name()),
        });
        functions
    }

    pub fn errors(&self) -> Vec<ErrorDefinition> {
        self.members().iter_error_definitions().collect()
    }

    pub fn compute_linearised_errors(&self) -> Vec<ErrorDefinition> {
        let mut errors = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            match base {
                ContractBase::Contract(contract) => {
                    errors.extend(contract.members().iter_error_definitions());
                }
                ContractBase::Interface(interface) => {
                    errors.extend(interface.members().iter_error_definitions());
                }
            }
        }
        errors
    }

    pub fn events(&self) -> Vec<EventDefinition> {
        self.members().iter_event_definitions().collect()
    }

    pub fn compute_linearised_events(&self) -> Vec<EventDefinition> {
        let mut events = Vec::new();
        let bases = self.compute_linearised_bases();
        for base in bases.iter().rev() {
            match base {
                ContractBase::Contract(contract) => {
                    events.extend(contract.members().iter_event_definitions());
                }
                ContractBase::Interface(interface) => {
                    events.extend(interface.members().iter_event_definitions());
                }
            }
        }
        events
    }
}

// ABI related extensions
impl ContractDefinitionStruct {
    // TODO: ideally the user wouldn't need to provide the file_id and we should
    // be able to obtain it here, but for that we need bi-directional tree
    // navigation
    pub fn compute_abi_with_file_id(&self, file_id: String) -> Option<ContractAbi> {
        let name = self.ir_node.name.unparse().to_string();
        let entries = self.compute_abi_entries()?;
        let (storage_layout, transient_storage_layout) = self.compute_storage_layout()?;
        Some(ContractAbi {
            node_id: self.ir_node.id(),
            name,
            file_id,
            entries,
            storage_layout,
            transient_storage_layout,
        })
    }

    fn compute_abi_entries(&self) -> Option<Vec<AbiEntry>> {
        let mut entries = Vec::new();
        if let Some(constructor) = self.constructor() {
            entries.push(constructor.compute_abi_entry()?);
        }
        for function in &self.compute_linearised_functions() {
            if function.is_externally_visible() {
                entries.push(function.compute_abi_entry()?);
            }
        }
        for state_variable in &self.compute_linearised_state_variables() {
            if state_variable.is_externally_visible() {
                entries.push(state_variable.compute_abi_entry()?);
            }
        }
        for error in &self.compute_linearised_errors() {
            entries.push(error.compute_abi_entry()?);
        }
        for event in &self.compute_linearised_events() {
            entries.push(event.compute_abi_entry()?);
        }

        entries.sort();
        Some(entries)
    }

    /// Retrieves the custom base slot for this contract, if specified. This is
    /// used for computing the base of the storage layout for non-transient
    /// state variables.
    fn base_slot(&self) -> Option<usize> {
        let binder::Definition::Contract(definition) = self
            .semantic
            .binder()
            .find_definition_by_id(self.ir_node.id())?
        else {
            unreachable!("definition is not a contract");
        };
        definition.base_slot
    }

    /// Computes the layouts of both permanent and transient state variables
    fn compute_storage_layout(&self) -> Option<(Vec<StorageItem>, Vec<StorageItem>)> {
        let all_state_variables = self.compute_linearised_state_variables();

        // TODO(validation): it is an error if any contract in the hierarchy
        // other than the leaf has a custom offset layout
        let storage_layout = self.lay_out_state_variables(
            self.base_slot().unwrap_or_default() * SemanticContext::SLOT_SIZE,
            all_state_variables.iter().filter(|state_variable| {
                matches!(
                    state_variable.mutability(),
                    StateVariableMutability::Mutable
                )
            }),
        )?;
        let transient_storage_layout = self.lay_out_state_variables(
            0usize,
            all_state_variables.iter().filter(|state_variable| {
                matches!(
                    state_variable.mutability(),
                    StateVariableMutability::Transient
                )
            }),
        )?;
        Some((storage_layout, transient_storage_layout))
    }

    fn lay_out_state_variables<'a>(
        &self,
        base_ptr: usize,
        variables: impl Iterator<Item = &'a StateVariableDefinition>,
    ) -> Option<Vec<StorageItem>> {
        let mut storage_layout = Vec::new();
        let mut ptr: usize = base_ptr;
        for state_variable in variables {
            let node_id = state_variable.ir_node.id();
            let variable_type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;

            // check if we can pack the variable in the previous slot
            let remaining_bytes = SemanticContext::SLOT_SIZE - (ptr % SemanticContext::SLOT_SIZE);
            if remaining_bytes < SemanticContext::SLOT_SIZE && variable_size > remaining_bytes {
                ptr += remaining_bytes;
            }

            let label = state_variable.ir_node.name.unparse().to_string();
            let slot = ptr.div(SemanticContext::SLOT_SIZE);
            let offset = ptr % SemanticContext::SLOT_SIZE;
            let r#type = self.semantic.type_internal_name(variable_type_id);
            storage_layout.push(StorageItem {
                node_id,
                label,
                slot,
                offset,
                r#type,
            });

            // ready pointer for the next variable
            ptr += variable_size;
        }
        Some(storage_layout)
    }
}

impl ContractMembersStruct {
    pub(crate) fn iter_function_definitions(
        &self,
    ) -> impl Iterator<Item = FunctionDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::FunctionDefinition(function) = member {
                Some(function)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_state_variable_definitions(
        &self,
    ) -> impl Iterator<Item = StateVariableDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::StateVariableDefinition(state_variable) = member {
                Some(state_variable)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_error_definitions(&self) -> impl Iterator<Item = ErrorDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::ErrorDefinition(error_definition) = member {
                Some(error_definition)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_event_definitions(&self) -> impl Iterator<Item = EventDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::EventDefinition(event_definition) = member {
                Some(event_definition)
            } else {
                None
            }
        })
    }
}

impl InterfaceMembersStruct {
    pub(crate) fn iter_error_definitions(&self) -> impl Iterator<Item = ErrorDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::ErrorDefinition(error_definition) = member {
                Some(error_definition)
            } else {
                None
            }
        })
    }

    pub(crate) fn iter_event_definitions(&self) -> impl Iterator<Item = EventDefinition> + use<'_> {
        self.iter().filter_map(|member| {
            if let ContractMember::EventDefinition(event_definition) = member {
                Some(event_definition)
            } else {
                None
            }
        })
    }
}
