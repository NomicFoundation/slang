use std::fmt::{self, Write as _};

use slang_solidity_v2_semantic::types::TypeId;

use crate::abi::AbiParameter;
use crate::abi::types::type_as_abi_type;
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
        self.write_canonical_signature(&mut signature)?;
        Some(signature)
    }

    pub(crate) fn compute_internal_signature(&self) -> Option<String> {
        let mut signature = String::new();
        self.write_parameter_names(&mut signature, |out, type_id| {
            out.write_str(&self.semantic.type_internal_name(type_id))
                .ok()
        })?;
        Some(signature)
    }

    pub(crate) fn write_canonical_signature<W: fmt::Write>(&self, out: &mut W) -> Option<()> {
        self.write_parameter_names(out, |out, type_id| {
            write!(out, "{}", type_as_abi_type(&self.semantic, type_id)?).ok()
        })
    }

    pub(crate) fn write_library_signature<W: fmt::Write>(&self, out: &mut W) -> Option<()> {
        self.write_parameter_names(out, |out, type_id| {
            out.write_str(&self.semantic.type_library_name(type_id)?)
                .ok()
        })
    }

    fn write_parameter_names<W: fmt::Write>(
        &self,
        out: &mut W,
        write_type: impl Fn(&mut W, TypeId) -> Option<()>,
    ) -> Option<()> {
        for (index, type_id) in self.parameter_types_iter().enumerate() {
            if index > 0 {
                out.write_char(',').ok()?;
            }
            write_type(out, type_id?)?;
        }
        Some(())
    }

    fn parameter_types_iter(&self) -> impl Iterator<Item = Option<TypeId>> + '_ {
        self.ir_nodes.iter().map(|parameter| {
            self.semantic
                .binder()
                .node_typing(parameter.id())
                .as_type_id()
        })
    }
}
