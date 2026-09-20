use anyhow::{Context, Result};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use solidity_v2_testing_utils::evm_targets::parse_evm_target_name;

/// `isoltest`'s placeholder for an EVM version that has not been released yet,
/// written in place of a target name (e.g. `EVMVersion: =@future`).
pub const FUTURE_EVM_VERSION: &str = "@future";

/// A successfully understood `EVMVersion` setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedTarget {
    /// The test asks for [`FUTURE_EVM_VERSION`], an EVM version that hasn't
    /// been released yet, so there is no target to analyze it at.
    FutureSpec,
    /// The concrete target to analyze the test at.
    Target(EvmTarget),
}

/// Resolves the `EVMVersion` setting (e.g. `>=byzantium`, `<constantinople`,
/// `istanbul`) to a single concrete [`EvmTarget`] to analyze the test at.
///
/// `default` is the EVM target `solc` of the language version being tested would
/// pick on its own (the runner passes the version's default). We prefer it
/// whenever it satisfies the constraint, so a lower-bound-only setting like
/// `>=byzantium` stays at the version's real default rather than jumping to the
/// newest target ever — which is a combination that `solc` of that version never
/// ran, and would spuriously enable target-gated builtins. Only when the default
/// doesn't satisfy the constraint (e.g. a hard `<constantinople` upper bound on a
/// newer default) do we pick the *nearest* satisfying target to the default.
///
/// When no setting is present, `default` is used.
///
/// [`ParsedTarget::FutureSpec`] means the test asks for an EVM version `slang`
/// has no target for at all. For now this only covers the [`FUTURE_EVM_VERSION`]
/// isoltest marker.
///
/// Anything else *is* an error: silently falling back on a setting we failed to
/// understand would analyze the test at the wrong target and quietly bake the
/// result into the baseline, so a setting `solc` accepts but we don't is treated
/// as a gap in *this* code.
pub fn resolve_evm_target(setting: Option<&str>, default: EvmTarget) -> Result<ParsedTarget> {
    let Some(setting) = setting else {
        return Ok(ParsedTarget::Target(default));
    };

    let setting = setting.trim();
    let (op, name) = if let Some(name) = setting.strip_prefix(">=") {
        (">=", name)
    } else if let Some(name) = setting.strip_prefix("<=") {
        ("<=", name)
    } else if let Some(name) = setting.strip_prefix('>') {
        (">", name)
    } else if let Some(name) = setting.strip_prefix('<') {
        ("<", name)
    } else if let Some(name) = setting.strip_prefix('=') {
        ("=", name)
    } else {
        ("=", setting)
    };

    let name = name.trim();

    // `isoltest`'s placeholder for the next, not-yet-released EVM version.
    if name == FUTURE_EVM_VERSION {
        return Ok(ParsedTarget::FutureSpec);
    }

    let bound = parse_evm_target_name(name)
        .with_context(|| format!("Unrecognized EVM target in 'EVMVersion: {setting}'."))?;

    let satisfies = |target: EvmTarget| match op {
        ">=" => target >= bound,
        ">" => target > bound,
        "<=" => target <= bound,
        "<" => target < bound,
        _ => target == bound,
    };

    // Prefer the version's own default target when it already satisfies the
    // constraint.
    if satisfies(default) {
        return Ok(ParsedTarget::Target(default));
    }

    // Otherwise pick the supported target satisfying the constraint that sits
    // closest to the default. Targets are declared oldest first and numbered
    // from zero, so the distance between two discriminants is how many forks
    // apart they are.
    EvmTarget::ALL
        .iter()
        .copied()
        .filter(|target| satisfies(*target))
        .min_by_key(|target| (*target as usize).abs_diff(default as usize))
        .map(ParsedTarget::Target)
        .with_context(|| format!("No supported EVM target satisfies 'EVMVersion: {setting}'."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_evm_version_constraints() {
        let resolve = |setting, default| match resolve_evm_target(setting, default).unwrap() {
            ParsedTarget::Target(target) => target,
            ParsedTarget::FutureSpec => panic!("'{setting:?}' should resolve to a concrete target"),
        };

        // No setting: the version's default target is used as-is.
        assert_eq!(resolve(None, EvmTarget::Istanbul), EvmTarget::Istanbul);

        // A lower bound the default already satisfies keeps the *version's*
        // default rather than jumping to the newest target ever — the crux of
        // the version-aware fix (a `>=byzantium` test at an Istanbul-default
        // version runs at Istanbul, not Amsterdam).
        assert_eq!(
            resolve(Some(">=byzantium"), EvmTarget::Istanbul),
            EvmTarget::Istanbul
        );

        // If the default is too old for the lower bound, pick the nearest
        // satisfying target above it.
        assert_eq!(
            resolve(Some(">=cancun"), EvmTarget::Istanbul),
            EvmTarget::Cancun
        );

        // Upper bounds the default violates resolve to the nearest satisfying
        // target below it.
        assert_eq!(
            resolve(Some("<constantinople"), EvmTarget::Osaka),
            EvmTarget::Byzantium
        );
        assert_eq!(
            resolve(Some("<=constantinople"), EvmTarget::Osaka),
            EvmTarget::Constantinople
        );

        // Exact constraints resolve to that target regardless of the default.
        assert_eq!(
            resolve(Some("=istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );
        assert_eq!(
            resolve(Some("istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );
    }

    #[test]
    fn yields_no_target_for_an_unreleased_evm_version() {
        // `@future` names an EVM version that doesn't exist yet, so there is
        // nothing to analyze the test at. That's reported as `FutureSpec` rather
        // than an error, because it says something about the test rather than
        // about this code — and the runner turns it into a failing test.
        for setting in ["=@future", "@future", ">=@future"] {
            assert_eq!(
                resolve_evm_target(Some(setting), EvmTarget::Istanbul).unwrap(),
                ParsedTarget::FutureSpec,
                "'{setting}' should resolve to no target"
            );
        }
    }

    #[test]
    fn rejects_evm_version_settings_it_cannot_honor() {
        // A name we don't know is a gap in *our* table: resolving it to the
        // default would analyze the test at a target `solc` never used.
        assert!(resolve_evm_target(Some("nonsense"), EvmTarget::Osaka).is_err());

        // Likewise a well-formed constraint no supported target satisfies.
        assert!(resolve_evm_target(Some(">amsterdam"), EvmTarget::Osaka).is_err());
    }
}
