// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (string memory, string memory, bytes memory) {
        // Only bytes that are valid UTF-8 convert to `string`.
        return (string(hex"41a000"), string(hex"c3a9"), bytes(hex"41a000"));
    }
}
