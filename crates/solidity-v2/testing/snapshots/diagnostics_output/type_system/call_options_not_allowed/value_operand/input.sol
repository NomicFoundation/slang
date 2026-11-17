// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint a) internal pure {
        // A value is not a function at all.
        a{gas: 1000};
    }
}
