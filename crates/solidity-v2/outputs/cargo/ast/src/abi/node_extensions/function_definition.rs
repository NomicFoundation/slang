use slang_solidity_v2_ir::ir;

use crate::abi::{
    selector_from_signature, AbiConstructor, AbiEntry, AbiFallback, AbiFunction, AbiMutability,
    AbiReceive,
};
use crate::ast::{Definition, FunctionDefinitionStruct, FunctionVisibility};

impl FunctionDefinitionStruct {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes().visibility(),
            FunctionVisibility::Public | FunctionVisibility::External
        )
    }

    pub fn compute_abi_entry(&self) -> Option<AbiEntry> {
        if !self.is_externally_visible() {
            return None;
        }
        let inputs = self.parameters().compute_abi_parameters()?;
        let outputs = if let Some(returns) = self.returns() {
            returns.compute_abi_parameters()?
        } else {
            Vec::new()
        };

        let node_id = self.ir_node.id();
        let name = self
            .ir_node
            .name
            .as_ref()
            .map(|name| name.unparse().to_string());
        let state_mutability: AbiMutability = (&self.ir_node.attributes.mutability).into();

        match self.ir_node.kind {
            ir::FunctionKind::Regular => Some(AbiEntry::Function(AbiFunction {
                node_id,
                name: name?,
                inputs,
                outputs,
                state_mutability,
            })),
            ir::FunctionKind::Constructor => Some(AbiEntry::Constructor(AbiConstructor {
                node_id,
                inputs,
                state_mutability,
            })),
            ir::FunctionKind::Fallback => Some(AbiEntry::Fallback(AbiFallback {
                node_id,
                state_mutability,
            })),
            ir::FunctionKind::Receive => Some(AbiEntry::Receive(AbiReceive {
                node_id,
                state_mutability,
            })),
            ir::FunctionKind::Modifier => None,
        }
    }

    /// Returns the external signature for this function, suitable for ABI encoding.
    ///
    /// This is only guaranteed for external functions in valid Solidity, as
    /// internal functions may contain parameter types that cannot be
    /// ABI-encoded.
    pub fn compute_canonical_signature(&self) -> Option<String> {
        let name = self.ir_node.name.as_ref()?.unparse();
        let parameters = self.parameters().compute_canonical_signature()?;
        Some(format!("{name}({parameters})"))
    }

    /// Returns the signature solc selects on for a library external function:
    /// internal (scope-qualified) type names, with a trailing ` storage` on
    /// storage-reference parameters. Unlike [`Self::compute_canonical_signature`],
    /// it is well-defined for storage references to non-ABI-encodable types.
    pub fn compute_library_signature(&self) -> Option<String> {
        let name = self.ir_node.name.as_ref()?.unparse();
        let parameters = self.parameters().compute_library_signature()?;
        Some(format!("{name}({parameters})"))
    }

    /// Returns the signature for this function using internal type names for
    /// parameters. Unlike [`Self::compute_canonical_signature`], this form is
    /// well-defined for any function, including internal ones with parameter
    /// types that cannot be ABI-encoded.
    pub fn compute_internal_signature(&self) -> Option<String> {
        let name = match self.ir_node.kind {
            ir::FunctionKind::Regular | ir::FunctionKind::Modifier => self
                .ir_node
                .name
                .as_ref()
                .expect("regular functions and modifiers must have a name")
                .unparse(),
            ir::FunctionKind::Constructor => "@constructor",
            ir::FunctionKind::Fallback => "fallback",
            ir::FunctionKind::Receive => "receive",
        };
        let parameters = self.parameters().compute_internal_signature()?;
        Some(format!("{name}({parameters})"))
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let signature = match self.enclosing_definition() {
            Some(Definition::Library(_)) => self.compute_library_signature()?,
            _ => self.compute_canonical_signature()?,
        };
        Some(selector_from_signature(&signature))
    }
}
