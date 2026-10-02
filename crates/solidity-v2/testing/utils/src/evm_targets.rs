use inflector::Inflector;
use slang_solidity_v2_common::evm_targets::EvmTarget;

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
    use infra_utils::solc::default_evm_version;
    use slang_solidity_v2_common::versions::LanguageVersion;

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
    fn default_evm_targets_match_solc() {
        for &version in LanguageVersion::ALL {
            assert_eq!(
                Some(version.default_evm_target()),
                parse_evm_target_name(default_evm_version(&version.into())),
                "{version}"
            );
        }
    }
}
