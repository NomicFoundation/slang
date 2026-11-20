// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(function(bytes calldata) external fp) public {
        fp("x");
    }
}
