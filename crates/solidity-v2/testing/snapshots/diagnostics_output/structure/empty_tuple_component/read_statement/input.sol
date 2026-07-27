// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // Invalid: a missing component in a read-position tuple.
        (1, , 2);
    }
}
