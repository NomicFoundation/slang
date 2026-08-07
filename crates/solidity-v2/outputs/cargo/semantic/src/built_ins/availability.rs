use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::built_ins::InternalBuiltIn;
use crate::built_ins::specifiers::built_in_specifiers;

/// Whether `built_in` exists as a usable built-in for the given language version
/// and EVM target.
///
/// Both axes have to allow it, since they gate independent things: a built-in the
/// compiler doesn't know about yet is just as unusable as one whose opcode the
/// target lacks. `clz`, for example, needs both 0.8.31 (when Solidity learned it)
/// and Osaka (when the opcode appears).
pub(crate) fn is_built_in_available(
    built_in: InternalBuiltIn,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
) -> bool {
    let (versions, targets) = built_in_specifiers(built_in);

    versions.is_none_or(|versions| versions.contains(language_version))
        && targets.is_none_or(|targets| targets.contains(evm_target))
}
