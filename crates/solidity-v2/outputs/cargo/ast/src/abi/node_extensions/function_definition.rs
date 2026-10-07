use std::fmt;

use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::AbiNameError;

use crate::abi::{
    AbiConstructor, AbiEntry, AbiFallback, AbiFunction, AbiMutability, AbiReceive, SignatureHasher,
    json_type_spelling,
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
                type_spelling: json_type_spelling(self.enclosing_definition().as_ref()),
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
        let mut signature = String::new();
        self.write_canonical_signature(self.ir_node.name.as_ref()?.unparse(), &mut signature)
            .ok()?;
        Some(signature)
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

    /// Returns the signature for this function using library type names for
    /// parameters: scope-qualified internal names, a trailing ` storage` on
    /// storage references, and a user-defined value type unwrapped to its
    /// underlying type — none of which the canonical form can spell.
    pub fn compute_library_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_library_signature(self.ir_node.name.as_ref()?.unparse(), &mut signature)
            .ok()?;
        Some(signature)
    }

    /// Returns the signature a selector is hashed from: the library form for a
    /// library member, the canonical form otherwise.
    pub fn compute_selector_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_selector_signature(self.ir_node.name.as_ref()?.unparse(), &mut signature)
            .ok()?;
        Some(signature)
    }

    pub fn compute_selector(&self) -> Option<u32> {
        if !self.is_externally_visible() {
            return None;
        }
        let mut hasher = SignatureHasher::default();
        self.write_selector_signature(self.ir_node.name.as_ref()?.unparse(), &mut hasher)
            .ok()?;
        Some(hasher.selector())
    }

    fn write_selector_signature(
        &self,
        name: &str,
        out: &mut impl fmt::Write,
    ) -> Result<(), AbiNameError> {
        match self.enclosing_definition() {
            Some(Definition::Library(_)) => self.write_library_signature(name, out),
            _ => self.write_canonical_signature(name, out),
        }
    }

    fn write_canonical_signature(
        &self,
        name: &str,
        out: &mut impl fmt::Write,
    ) -> Result<(), AbiNameError> {
        write!(out, "{name}(")?;
        self.parameters().write_canonical_signature(out)?;
        Ok(out.write_char(')')?)
    }

    fn write_library_signature(
        &self,
        name: &str,
        out: &mut impl fmt::Write,
    ) -> Result<(), AbiNameError> {
        write!(out, "{name}(")?;
        self.parameters().write_library_signature(out)?;
        Ok(out.write_char(')')?)
    }
}
