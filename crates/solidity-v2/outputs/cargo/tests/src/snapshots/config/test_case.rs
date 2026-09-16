use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

/// One cell of a snapshot test's run: the language version to analyze at, the
/// EVM target to analyze it at, and whether `solc` is expected to disagree
/// there.
#[derive(Clone, Copy)]
pub struct TestCase {
    /// The language version to analyze at.
    pub language_version: LanguageVersion,

    /// The EVM target to analyze at.
    pub evm_target: EvmTarget,

    /// Whether slang and solc are expected to disagree on the status
    /// (success/failure) of this version's snapshot.
    pub expected_solc_divergence: bool,
}
