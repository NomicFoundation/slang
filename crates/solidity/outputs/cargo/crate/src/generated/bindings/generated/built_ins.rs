// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

// TODO: This should be moved to the Solidity-specific 'extensions' sub-module.
#[allow(unused_variables)]
pub fn get_built_ins_contents(version: &Version) -> &'static str {
    if *version < Version::new(0, 4, 17) {
        include_str!("./built_ins/0.4.11.sol")
    } else if *version < Version::new(0, 4, 22) {
        include_str!("./built_ins/0.4.17.sol")
    } else if *version < Version::new(0, 5, 0) {
        include_str!("./built_ins/0.4.22.sol")
    } else if *version < Version::new(0, 5, 3) {
        include_str!("./built_ins/0.5.0.sol")
    } else if *version < Version::new(0, 6, 0) {
        include_str!("./built_ins/0.5.3.sol")
    } else if *version < Version::new(0, 6, 2) {
        include_str!("./built_ins/0.6.0.sol")
    } else if *version < Version::new(0, 6, 7) {
        include_str!("./built_ins/0.6.2.sol")
    } else if *version < Version::new(0, 6, 8) {
        include_str!("./built_ins/0.6.7.sol")
    } else if *version < Version::new(0, 7, 0) {
        include_str!("./built_ins/0.6.8.sol")
    } else if *version < Version::new(0, 8, 0) {
        include_str!("./built_ins/0.7.0.sol")
    } else if *version < Version::new(0, 8, 2) {
        include_str!("./built_ins/0.8.0.sol")
    } else if *version < Version::new(0, 8, 4) {
        include_str!("./built_ins/0.8.2.sol")
    } else if *version < Version::new(0, 8, 7) {
        include_str!("./built_ins/0.8.4.sol")
    } else if *version < Version::new(0, 8, 8) {
        include_str!("./built_ins/0.8.7.sol")
    } else if *version < Version::new(0, 8, 11) {
        include_str!("./built_ins/0.8.8.sol")
    } else if *version < Version::new(0, 8, 18) {
        include_str!("./built_ins/0.8.11.sol")
    } else if *version < Version::new(0, 8, 24) {
        include_str!("./built_ins/0.8.18.sol")
    } else if *version < Version::new(0, 8, 26) {
        include_str!("./built_ins/0.8.24.sol")
    } else {
        include_str!("./built_ins/0.8.26.sol")
    }
}
