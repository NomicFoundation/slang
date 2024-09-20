// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

#[allow(unused_variables)]
pub fn get_contents(version: &Version) -> &'static str {
    if *version < Version::new(0, 5, 0) {
        r####"contract $$ {
function addmod(uint x, uint y, uint k) public returns (uint);
function assert(bool condition) public;
function revert(string memory reason) public;
struct $BuiltIn$Address {
  uint256 balance;
}
struct $BuiltIn$TxType {
  uint gasprice;
  address payable origin;
}
uint now;
$BuiltIn$TxType tx;
}"####
    } else if *version < Version::new(0, 8, 0) {
        r####"contract $$ {
function addmod(uint x, uint y, uint k) public returns (uint);
function assert(bool condition) public;
function require(bool condition) public;
function require(bool condition, string memory message) public;
function revert(string memory reason) public;
struct $BuiltIn$Address {
  uint256 balance;
}
struct $BuiltIn$TxType {
  uint gasprice;
  address payable origin;
}
uint now;
$BuiltIn$TxType tx;
}"####
    } else {
        r####"contract $$ {
function addmod(uint x, uint y, uint k) public returns (uint);
function assert(bool condition) public;
function require(bool condition) public;
function require(bool condition, string memory message) public;
function revert(string memory reason) public;
struct $BuiltIn$Address {
  uint256 balance;
  bytes code;
}
struct $BuiltIn$TxType {
  uint gasprice;
  address payable origin;
}
uint now;
$BuiltIn$TxType tx;
}"####
    }
}
