use inflector::Inflector;
use infra_utils::solc::default_evm_version;
use semver::Version;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

/// The EVM target `solc` of the given language version defaults to when no
/// explicit target is specified.
pub fn default_evm_target(language_version: LanguageVersion) -> EvmTarget {
    let version: Version = language_version.into();
    let name = default_evm_version(&version);

    parse_evm_target_name(name).unwrap_or_else(|| {
        panic!("'{name}' is the default EVM version of {version}, but is not a known EVM target.")
    })
}

/// Maps an `evmVersion` name as written by `solc` (camelCase, e.g.
/// `tangerineWhistle`) to the corresponding [`EvmTarget`], whose own `Display`
/// is `PascalCase`.
pub fn parse_evm_target_name(name: &str) -> Option<EvmTarget> {
    EvmTarget::ALL
        .iter()
        .copied()
        .find(|target| target.to_string().to_camel_case() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_solc_evm_version_names() {
        assert_eq!(
            parse_evm_target_name("tangerineWhistle"),
            Some(EvmTarget::TangerineWhistle)
        );
        assert_eq!(parse_evm_target_name("istanbul"), Some(EvmTarget::Istanbul));
        // `solc` writes these in camelCase, and nothing else is a name it uses.
        assert_eq!(parse_evm_target_name("Istanbul"), None);
        assert_eq!(parse_evm_target_name("TangerineWhistle"), None);
        assert_eq!(parse_evm_target_name("nonesuch"), None);
    }

    #[test]
    fn every_supported_version_has_a_default_target() {
        for &version in LanguageVersion::ALL {
            assert!(matches!(default_evm_target(version), _), "{version}");
        }
    }
}
