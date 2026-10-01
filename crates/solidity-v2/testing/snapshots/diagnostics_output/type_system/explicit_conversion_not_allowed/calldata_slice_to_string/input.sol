// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure returns (string memory) {
        return string(b[1:]);
    }
}
