// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 n) public pure {
        assembly {
            for { function g() {} } n { } { }
        }
    }
}
