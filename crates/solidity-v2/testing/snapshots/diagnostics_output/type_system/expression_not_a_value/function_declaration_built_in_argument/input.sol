// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `abi.encodeCall` takes the function it encodes a call to, and one
// reached through a contract type name is a declaration rather than a value.

contract C {
    function g(uint256 x) external {}
}

contract Test {
    function f() public pure returns (bytes memory) {
        return abi.encodeCall(C.g, (1));
    }
}
