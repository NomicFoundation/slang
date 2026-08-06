// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 n) public pure {
        assembly {
            // The function is in the init block of the inner for-loop, which is
            // in the post block of the outer for-loop. The inner init block is
            // what matters, so this is still an error.
            for { } n { for { function g() {} } n { } { } } { }
        }
    }
}
