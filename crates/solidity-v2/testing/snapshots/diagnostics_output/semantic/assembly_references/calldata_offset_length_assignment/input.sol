// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure {
        assembly {
            b.offset := 0
            b.length := 0
        }
    }
}
