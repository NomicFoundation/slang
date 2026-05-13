use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum UserFacingV1Crate {
    // Sorted by dependency order (from dependencies to dependents):
    metaslang_cst,
    metaslang_graph_builder,
    metaslang_stack_graphs,
    metaslang_bindings,
    slang_solidity,
    slang_solidity_cli,
}

impl UserFacingV1Crate {
    pub fn has_library_target(self) -> bool {
        match self {
            Self::metaslang_cst => true,
            Self::metaslang_graph_builder => true,
            Self::metaslang_stack_graphs => true,
            Self::metaslang_bindings => true,
            Self::slang_solidity => true,
            Self::slang_solidity_cli => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum UserFacingV2Crate {
    // Sorted by dependency order (from dependencies to dependents):
    slang_solidity_v2_common,
    slang_solidity_v2_cst,
    slang_solidity_v2_parser,
    slang_solidity_v2_ir,
    slang_solidity_v2_semantic,
    slang_solidity_v2_ast,
    slang_solidity_v2,
}

impl UserFacingV2Crate {
    pub fn has_library_target(self) -> bool {
        match self {
            Self::slang_solidity_v2_common => true,
            Self::slang_solidity_v2_cst => true,
            Self::slang_solidity_v2_parser => true,
            Self::slang_solidity_v2_ir => true,
            Self::slang_solidity_v2_semantic => true,
            Self::slang_solidity_v2_ast => true,
            Self::slang_solidity_v2 => true,
        }
    }
}
