use std::cmp::Ordering;
use std::ops::Div;

use sha3::{Digest, Keccak256};

use super::binder::Definition;
use super::ir::ast::{
    ContractDefinitionStruct, ErrorDefinitionStruct, EventDefinitionStruct,
    FunctionDefinitionStruct, FunctionVisibility, ParametersStruct, StateVariableDefinitionStruct,
    StateVariableMutability, StateVariableVisibility,
};
use super::ir::ir2_flat_contracts as input_ir;
use super::types::{Type, TypeId};
use super::SemanticAnalysis;
use crate::cst::NodeId;

pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub entries: Vec<AbiEntry>,
    pub storage_layout: Vec<StorageItem>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbiMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl From<input_ir::FunctionMutability> for AbiMutability {
    fn from(value: input_ir::FunctionMutability) -> Self {
        match value {
            input_ir::FunctionMutability::Pure => Self::Pure,
            input_ir::FunctionMutability::View => Self::View,
            input_ir::FunctionMutability::NonPayable => Self::NonPayable,
            input_ir::FunctionMutability::Payable => Self::Payable,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AbiEntry {
    Constructor {
        node_id: NodeId,
        inputs: Vec<AbiParameter>,
        state_mutability: AbiMutability,
    },
    Error {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
    },
    Event {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
        anonymous: bool,
    },
    Fallback {
        node_id: NodeId,
        state_mutability: AbiMutability,
    },
    Function {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
        outputs: Vec<AbiParameter>,
        state_mutability: AbiMutability,
    },
    Receive {
        node_id: NodeId,
        state_mutability: AbiMutability,
    },
}

impl PartialEq for AbiEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Constructor {
                    node_id: self_node_id,
                    ..
                },
                Self::Constructor {
                    node_id: other_node_id,
                    ..
                },
            )
            | (
                Self::Error {
                    node_id: self_node_id,
                    ..
                },
                Self::Error {
                    node_id: other_node_id,
                    ..
                },
            )
            | (
                Self::Event {
                    node_id: self_node_id,
                    ..
                },
                Self::Event {
                    node_id: other_node_id,
                    ..
                },
            )
            | (
                Self::Fallback {
                    node_id: self_node_id,
                    ..
                },
                Self::Fallback {
                    node_id: other_node_id,
                    ..
                },
            )
            | (
                Self::Function {
                    node_id: self_node_id,
                    ..
                },
                Self::Function {
                    node_id: other_node_id,
                    ..
                },
            )
            | (
                Self::Receive {
                    node_id: self_node_id,
                    ..
                },
                Self::Receive {
                    node_id: other_node_id,
                    ..
                },
            ) => self_node_id == other_node_id,
            _ => false,
        }
    }
}

impl Eq for AbiEntry {}

impl Ord for AbiEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Constructor { .. }, Self::Constructor { .. })
            | (Self::Fallback { .. }, Self::Fallback { .. })
            | (Self::Receive { .. }, Self::Receive { .. }) => Ordering::Equal,
            (
                Self::Error {
                    name: self_name, ..
                },
                Self::Error {
                    name: other_name, ..
                },
            )
            | (
                Self::Event {
                    name: self_name, ..
                },
                Self::Event {
                    name: other_name, ..
                },
            )
            | (
                Self::Function {
                    name: self_name, ..
                },
                Self::Function {
                    name: other_name, ..
                },
            ) => self_name.cmp(other_name),

            (Self::Constructor { .. }, _) => Ordering::Less,
            (_, Self::Constructor { .. }) => Ordering::Greater,
            (Self::Error { .. }, _) => Ordering::Less,
            (_, Self::Error { .. }) => Ordering::Greater,
            (Self::Event { .. }, _) => Ordering::Less,
            (_, Self::Event { .. }) => Ordering::Greater,
            (Self::Fallback { .. }, _) => Ordering::Less,
            (_, Self::Fallback { .. }) => Ordering::Greater,
            (Self::Function { .. }, _) => Ordering::Less,
            (_, Self::Function { .. }) => Ordering::Greater,
        }
    }
}

impl PartialOrd for AbiEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterComponent {
    pub name: String,
    pub r#type: String,
    pub components: Vec<ParameterComponent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbiParameter {
    pub node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    pub name: Option<String>,
    pub r#type: String,
    pub components: Vec<ParameterComponent>,
    pub indexed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItem {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}

impl ContractDefinitionStruct {
    // TODO: ideally the user wouldn't need to provide the file_id and we should
    // be able to obtain it here, but for that we need bi-directional tree
    // navigation
    pub fn compute_abi_with_file_id(&self, file_id: String) -> Option<ContractAbi> {
        let name = self.ir_node.name.unparse();
        let entries = self.compute_abi_entries()?;
        let storage_layout = self.compute_storage_layout()?;
        Some(ContractAbi {
            node_id: self.ir_node.node_id,
            name,
            file_id,
            entries,
            storage_layout,
        })
    }

    fn compute_abi_entries(&self) -> Option<Vec<AbiEntry>> {
        let mut entries = Vec::new();
        if let Some(constructor) = self.constructor() {
            entries.push(constructor.compute_abi()?);
        }
        for function in &self.compute_linearised_functions() {
            if function.is_externally_visible() {
                entries.push(function.compute_abi()?);
            }
        }
        for state_variable in &self.compute_linearised_state_variables() {
            if state_variable.is_externally_visible() {
                entries.push(state_variable.compute_abi()?);
            }
        }
        for error in &self.compute_linearised_errors() {
            entries.push(error.compute_abi()?);
        }
        for event in &self.compute_linearised_events() {
            entries.push(event.compute_abi()?);
        }

        entries.sort();
        Some(entries)
    }

    fn compute_storage_layout(&self) -> Option<Vec<StorageItem>> {
        let mut storage_layout = Vec::new();
        // TODO: if the contract has a specific storage layout specifier, we
        // need to compute its value and use it as the base
        let mut ptr: usize = 0;
        for state_variable in &self.compute_linearised_state_variables() {
            let node_id = state_variable.ir_node.node_id;
            // skip constants and immutable variables, since they are not placed in storage
            // TODO: also, transient storage is laid out separately and we need
            // to support that as well
            if !matches!(
                state_variable.mutability(),
                StateVariableMutability::Mutable
            ) {
                continue;
            }

            let variable_type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let variable_size = self.semantic.storage_size_of_type_id(variable_type_id)?;

            // check if we can pack the variable in the previous slot
            let remaining_bytes = SemanticAnalysis::SLOT_SIZE - (ptr % SemanticAnalysis::SLOT_SIZE);
            if remaining_bytes < SemanticAnalysis::SLOT_SIZE && variable_size > remaining_bytes {
                ptr += remaining_bytes;
            }

            let label = state_variable.ir_node.name.unparse();
            let slot = ptr.div(SemanticAnalysis::SLOT_SIZE);
            let offset = ptr % SemanticAnalysis::SLOT_SIZE;
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

impl FunctionDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.visibility(),
            FunctionVisibility::Public | FunctionVisibility::External
        )
    }

    pub fn compute_abi(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let inputs = self.parameters().compute_abi()?;
        let outputs = if let Some(returns) = self.returns() {
            returns.compute_abi()?
        } else {
            Vec::new()
        };

        let node_id = self.ir_node.node_id;
        let name = self.ir_node.name.as_ref().map(|name| name.unparse());
        let state_mutability: AbiMutability = self.ir_node.mutability.clone().into();

        match self.ir_node.kind {
            input_ir::FunctionKind::Regular => Some(AbiEntry::Function {
                node_id,
                name: name?,
                inputs,
                outputs,
                state_mutability,
            }),
            input_ir::FunctionKind::Constructor => Some(AbiEntry::Constructor {
                node_id,
                inputs,
                state_mutability,
            }),
            input_ir::FunctionKind::Unnamed | input_ir::FunctionKind::Fallback => {
                Some(AbiEntry::Fallback {
                    node_id,
                    state_mutability,
                })
            }
            input_ir::FunctionKind::Receive => Some(AbiEntry::Receive {
                node_id,
                state_mutability,
            }),
            input_ir::FunctionKind::Modifier => None,
        }
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let name = self.ir_node.name.as_ref()?.unparse();
        let signature = format!(
            "{name}({parameters})",
            parameters = self.parameters().compute_canonical_signature()?,
        );

        Some(selector_from_signature(&signature))
    }
}

impl ParametersStruct {
    pub(crate) fn compute_abi(&self) -> Option<Vec<AbiParameter>> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.node_id;
            let name = parameter.name.as_ref().map(|name| name.unparse());
            let indexed = parameter.indexed;
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            let (r#type, components) = self.semantic.type_as_abi_parameter(type_id)?;
            result.push(AbiParameter {
                node_id: Some(node_id),
                name,
                r#type,
                components,
                indexed,
            });
        }
        Some(result)
    }

    pub(crate) fn compute_canonical_signature(&self) -> Option<String> {
        let mut result = Vec::new();
        for parameter in &self.ir_nodes {
            let node_id = parameter.node_id;
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder.node_typing(node_id).as_type_id()?;
            result.push(self.semantic.type_canonical_name(type_id)?);
        }
        Some(result.join(","))
    }
}

impl StateVariableDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(self.visibility(), StateVariableVisibility::Public)
    }

    fn extract_getter_type_parameters_abi(&self) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let Definition::StateVariable(definition) = self
            .semantic
            .binder
            .find_definition_by_id(self.ir_node.node_id)?
        else {
            unreachable!("definition is not a state variable");
        };
        self.semantic
            .extract_function_type_parameters_abi(definition.getter_type_id?)
    }

    pub fn compute_abi(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, outputs) = self.extract_getter_type_parameters_abi()?;

        Some(AbiEntry::Function {
            node_id: self.ir_node.node_id,
            name: self.ir_node.name.unparse(),
            inputs,
            outputs,
            state_mutability: AbiMutability::View,
        })
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let (inputs, _) = self.extract_getter_type_parameters_abi()?;

        let signature = format!(
            "{name}({parameters})",
            name = self.ir_node.name.unparse(),
            parameters = inputs
                .into_iter()
                .map(|parameter| parameter.r#type)
                .collect::<Vec<_>>()
                .join(","),
        );

        Some(selector_from_signature(&signature))
    }
}

impl SemanticAnalysis {
    fn extract_function_type_parameters_abi(
        &self,
        type_id: TypeId,
    ) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
        let Type::Function(function_type) = self.types.get_type_by_id(type_id) else {
            return None;
        };
        let mut inputs = Vec::new();
        for parameter_type_id in &function_type.parameter_types {
            let (r#type, components) = self.type_as_abi_parameter(*parameter_type_id)?;
            inputs.push(AbiParameter {
                node_id: None,
                name: None,
                r#type,
                components,
                indexed: false,
            });
        }
        let (r#type, components) = self.type_as_abi_parameter(function_type.return_type)?;
        let outputs = vec![AbiParameter {
            node_id: None,
            name: None,
            r#type,
            components,
            indexed: false,
        }];
        Some((inputs, outputs))
    }

    fn type_as_abi_parameter(&self, type_id: TypeId) -> Option<(String, Vec<ParameterComponent>)> {
        match self.types.get_type_by_id(type_id) {
            Type::Array { element_type, .. } => {
                let (element_type_name, element_components) =
                    self.type_as_abi_parameter(*element_type)?;
                Some((format!("{element_type_name}[]"), element_components))
            }
            Type::Struct { definition_id, .. } => {
                // We need to recursively expand the struct fields as components
                let Definition::Struct(definition) = self
                    .binder
                    .find_definition_by_id(*definition_id)
                    .expect("the definition of a type exists")
                else {
                    unreachable!("the definition of a struct type is a struct");
                };
                let mut components = Vec::new();
                for member in &definition.ir_node.members {
                    let name = member.name.unparse();
                    let member_type_id = self.binder.node_typing(member.node_id).as_type_id()?;
                    let (r#type, subcomponents) = self.type_as_abi_parameter(member_type_id)?;
                    components.push(ParameterComponent {
                        name,
                        r#type,
                        components: subcomponents,
                    });
                }
                Some(("tuple".to_string(), components))
            }
            _ => Some((self.type_canonical_name(type_id)?, Vec::new())),
        }
    }
}

fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let result = hasher.finalize();

    let selector_bytes: [u8; 4] = result[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}

impl ErrorDefinitionStruct {
    pub fn compute_abi(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi()?;

        Some(AbiEntry::Error {
            node_id: self.ir_node.node_id,
            name: self.ir_node.name.unparse(),
            inputs,
        })
    }
}

impl EventDefinitionStruct {
    pub fn compute_abi(&self) -> Option<AbiEntry> {
        let inputs = self.parameters().compute_abi()?;

        Some(AbiEntry::Event {
            node_id: self.ir_node.node_id,
            name: self.ir_node.name.unparse(),
            inputs,
            anonymous: self.ir_node.anonymous_keyword,
        })
    }
}
