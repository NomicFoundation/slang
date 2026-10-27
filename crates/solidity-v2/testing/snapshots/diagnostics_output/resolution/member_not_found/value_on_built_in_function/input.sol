// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/nameAndTypeResolution/502_builtin_keccak256_reject_value.sol
        // Built-in functions are not payable, so they have no `value` member.
        keccak256.value();
    }
}
