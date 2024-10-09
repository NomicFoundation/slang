// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

#[allow(unused_variables)]
pub fn get_contents(version: &Version) -> &'static str {
    if *version < Version::new(0, 5, 0) {
        include_str!("./built_ins/0.4.11.sol")
    } else if *version < Version::new(0, 8, 0) {
        include_str!("./built_ins/0.5.0.sol")
    } else {
        include_str!("./built_ins/0.8.0.sol")
    }
}
