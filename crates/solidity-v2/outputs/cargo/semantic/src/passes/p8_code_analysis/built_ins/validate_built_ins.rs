use std::ops::Range;

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    IncompatibleBuiltInTarget, IncompatibleBuiltInVersion,
};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::built_ins::{InternalBuiltIn, built_in_specifiers};
use crate::context::FileNodeMapper;

pub(super) struct BuiltInValidator<'a> {
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
}

impl<'a> BuiltInValidator<'a> {
    pub(super) fn new(
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
    ) -> Self {
        Self {
            language_version,
            evm_target,
            file_node_mapper,
            diagnostics,
        }
    }

    /// Reports a reference to a built-in that isn't compatible with the
    /// configured language version or EVM target.
    ///
    /// The two axes are reported independently, each carrying the range it is
    /// compatible with, so a reference that is wrong on both produces one
    /// diagnostic per axis.
    pub(super) fn validate(
        &mut self,
        built_in: InternalBuiltIn,
        node_id: NodeId,
        range: &Range<usize>,
    ) {
        let (versions, targets) = built_in_specifiers(built_in);

        if let Some(compatible_in) = versions
            && !compatible_in.contains(self.language_version)
        {
            self.report(node_id, range, IncompatibleBuiltInVersion { compatible_in });
        }

        if let Some(compatible_in) = targets
            && !compatible_in.contains(self.evm_target)
        {
            self.report(node_id, range, IncompatibleBuiltInTarget { compatible_in });
        }
    }

    fn report(
        &mut self,
        node_id: NodeId,
        range: &Range<usize>,
        diagnostic: impl Into<DiagnosticKind>,
    ) {
        let file_id = self.file_node_mapper.file_id_from_node_id(node_id);
        self.diagnostics
            .push(file_id.to_owned(), range.clone(), diagnostic);
    }
}
