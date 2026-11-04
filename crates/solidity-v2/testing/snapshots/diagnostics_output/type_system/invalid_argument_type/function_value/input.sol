// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(function(uint256) internal f) internal {
        f(true);
    }
}
