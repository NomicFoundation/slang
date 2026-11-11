// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(function(bytes calldata) internal fp) internal {
        fp("x");
    }
}
