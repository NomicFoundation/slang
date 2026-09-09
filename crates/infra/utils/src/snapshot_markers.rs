//! Stable placeholders for the parts of a rendered diagnostic that vary with
//! the version, the EVM target, or the machine a snapshot test runs on.
//!
//! Not only this eliminates hundreds of redundant/duplicate snapshots,
//! it also makes it possible to run tests cross-platform, without failing
//! because the the OS/arch is different.
//!
//! Only the value the test is currently running is replaced. A mention of any
//! other release (the version a feature was introduced in, say) identifies that
//! release rather than the run, and is left as-is.

use regex::Regex;

/// Replaces the `LanguageVersion` slang is compiling with, as slang spells it:
/// `0.8.19`.
pub const CURRENT_SLANG_LANGUAGE_VERSION: &str = "__CURRENT_SLANG_LANGUAGE_VERSION__";

/// Replaces the `EvmTarget` slang is compiling for, as slang spells it, in
/// Pascal case: `SpuriousDragon`.
pub const CURRENT_SLANG_EVM_TARGET: &str = "__CURRENT_SLANG_EVM_TARGET__";

/// Replaces the full build description of the `solc` binary being run:
/// `0.8.19+commit.7dd6d404.Linux.g++`. It embeds both the version and the host
/// platform, so it churns between versions, and differs between dev machines.
pub const CURRENT_SOLC_BUILD_VERSION: &str = "__CURRENT_SOLC_BUILD_VERSION__";

/// Replaces the EVM version `solc` is compiling for, as `solc` spells it, in
/// camel case: `spuriousDragon`.
pub const CURRENT_SOLC_EVM_VERSION: &str = "__CURRENT_SOLC_EVM_VERSION__";

/// Replaces every match of `pattern` in `message` with `marker`. `pattern` is
/// built around the `test_value` the test is currently running, so that a mention of
/// any other release is left alone.
///
/// Compiling `pattern` costs ~200x more than running it, so a message that
/// doesn't contain `test_value` verbatim skips it. Most diagnostics never
/// mention the version or target at all, and this is what keeps a snapshot
/// run from paying for a `Regex` per rendered diagnostic.
pub fn replace_marker(message: &str, test_value: &str, pattern: &str, marker: &str) -> String {
    if !message.contains(test_value) {
        return message.to_owned();
    }

    assert!(
        pattern.contains(&regex::escape(test_value)),
        "Pattern '{pattern}' must contain the test value '{test_value}'"
    );

    Regex::new(pattern)
        .unwrap()
        .replace_all(message, marker)
        .into_owned()
}
