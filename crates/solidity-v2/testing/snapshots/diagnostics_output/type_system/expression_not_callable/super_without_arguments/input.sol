// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/conversion/convert_to_super_empty.sol
        // Calling `super`, which names the base contracts and is not a callable.
        super().x;
    }
}
