// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

contract C {
    function f(address a) public pure {
        // An address converts to a library type.
        L(a);
    }
}
