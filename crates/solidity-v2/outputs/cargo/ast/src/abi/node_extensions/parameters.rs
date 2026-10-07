use std::fmt;

use slang_solidity_v2_semantic::context::{AbiNameError, AbiTypeSpelling};
use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::AbiParameter;
use crate::ast::ParametersStruct;

impl ParametersStruct {
    pub(crate) fn compute_abi_parameters(&self) -> Option<Vec<AbiParameter>> {
        let mut result = Vec::with_capacity(self.ir_nodes.len());
        for parameter in self.ir_nodes.iter() {
            let node_id = parameter.id();
            let name = parameter
                .name
                .as_ref()
                .map(|name| name.unparse().to_string());
            // Bail out with `None` if any of the parameters fails typing
            let type_id = self.semantic.binder().node_typing(node_id).as_type_id()?;
            result.push(AbiParameter::new(
                Some(node_id),
                name,
                type_id,
                parameter.is_indexed,
                &self.semantic,
            )?);
        }
        Some(result)
    }

    pub(crate) fn compute_canonical_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_canonical_signature(&mut signature).ok()?;
        Some(signature)
    }

    pub(crate) fn compute_internal_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_parameter_names(&mut signature, |out, type_id| {
            Ok(self.semantic.write_type_internal_name(type_id, out)?)
        })
        .ok()?;
        Some(signature)
    }

    pub(crate) fn write_canonical_signature<W: fmt::Write>(
        &self,
        out: &mut W,
    ) -> Result<(), AbiNameError> {
        self.write_abi_signature(AbiTypeSpelling::Selector, out)
    }

    pub(crate) fn write_library_signature<W: fmt::Write>(
        &self,
        out: &mut W,
    ) -> Result<(), AbiNameError> {
        self.write_abi_signature(AbiTypeSpelling::LibrarySelector, out)
    }

    fn write_abi_signature<W: fmt::Write>(
        &self,
        spelling: AbiTypeSpelling,
        out: &mut W,
    ) -> Result<(), AbiNameError> {
        self.write_parameter_names(out, |out, type_id| {
            self.semantic.write_type_abi_name(type_id, spelling, out)
        })
    }

    fn write_parameter_names<W: fmt::Write>(
        &self,
        out: &mut W,
        write_type: impl Fn(&mut W, TypeId) -> Result<(), AbiNameError>,
    ) -> Result<(), AbiNameError> {
        for (index, parameter) in self.ir_nodes.iter().enumerate() {
            if index > 0 {
                out.write_char(',')?;
            }
            let type_id = self
                .semantic
                .binder()
                .node_typing(parameter.id())
                .as_type_id()
                .ok_or(AbiNameError::Unresolved(parameter.id()))?;
            write_type(out, type_id)?;
        }
        Ok(())
    }
}
