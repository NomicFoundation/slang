// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8) internal pure {}

    function f(bool) internal pure {}

    function no_overload_accepts() internal pure {
        // A string converts to neither parameter type.
        f("nope");
    }

    function one_overload_accepts() internal pure {
        f(true);
    }
}
