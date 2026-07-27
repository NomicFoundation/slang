// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 n) public pure {
        assembly {
            // The function is in the init block of the inner for-loop, which is
            // itself in the init block of the outer for-loop.
            for { for { function g() {} } n { } { } } n { } { }
        }
    }
}
