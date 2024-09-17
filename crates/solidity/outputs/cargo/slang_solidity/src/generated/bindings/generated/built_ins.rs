// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

pub fn get_contents(_version: &Version) -> &'static str {
    r#####"
    library $$ {
function assert();
function require();
function revert();
struct $builtin$Address {}
struct $builtin$TxType {}
uint now;
$builtin$TxType tx;
}
"#####
}
