// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

#[allow(unused_variables)]
pub fn get_contents(version: &Version) -> &'static str {
    if *version < Version::new(0, 5, 0) {
        r####"library $$ {
function assert();
function revert();
struct $builtin$Address {}
struct $builtin$TxType {}
uint now;
$builtin$TxType tx;
}"####
    } else {
        r####"library $$ {
function assert();
function require();
function revert();
struct $builtin$Address {}
struct $builtin$TxType {}
uint now;
$builtin$TxType tx;
}"####
    }
}
