use semver::Version;

/// Returns true if a given `solc` version is known to segfault on a multitude
/// of parse errors and should be skipped from automated testing.
pub fn is_solc_segfault(version: &Version) -> bool {
    matches!(
        version,
        Version {
            major: 0,
            minor: 4,
            patch: 11,
            pre: _,
            build: _,
        }
    )
}

/// Returns the default `evmVersion` that `solc` would
/// pick when no explicit `evmVersion` is set in the project's compiler
/// settings, for the given Solidity compiler version.
///
/// Note: it uses the same 'camelCase' casing as 'solc' JSON API.
pub fn default_evm_version(version: &Version) -> &'static str {
    match (version.major, version.minor, version.patch) {
        (0, 4, 11..=20) => {
            panic!("solc version '{version}' didn't allow specifying the EVM version.")
        }

        (0, 4, 21..=26) => "byzantium",

        (0, 5, 0..=4) => "byzantium",
        (0, 5, 5..=13) => "petersburg",
        (0, 5, 14..) => "istanbul",

        (0, 6, _) => "istanbul",

        (0, 7, _) => "istanbul",

        (0, 8, 0..=4) => "istanbul",
        (0, 8, 5..=6) => "berlin",
        (0, 8, 7..=17) => "london",
        (0, 8, 18..=19) => "paris",
        (0, 8, 20..=24) => "shanghai",
        (0, 8, 25..=29) => "cancun",
        (0, 8, 30) => "prague",
        (0, 8, 31..=36) => "osaka",

        _ => panic!(
            "Unrecognized solc version '{version}'. Please add it to `default_evm_version()`."
        ),
    }
}
