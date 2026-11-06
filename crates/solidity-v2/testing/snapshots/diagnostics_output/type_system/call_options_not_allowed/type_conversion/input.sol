// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {}

contract C {
    function f(address a) internal pure {
        // A conversion through a contract name is not a message call.
        D{value: 1}(a);
    }
}
