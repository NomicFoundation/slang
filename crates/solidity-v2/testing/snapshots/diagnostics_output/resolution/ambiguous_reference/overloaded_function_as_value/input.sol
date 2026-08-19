// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() internal pure {}

    function f(uint256) internal pure {}

    function g() internal pure {}

    function takes(function() internal pure) internal pure {}

    function ambiguous() internal pure {
        // The overload set is passed as a value rather than called, so the
        // arguments of an enclosing call can't narrow it down.
        takes(f);
    }

    function unambiguous() internal pure {
        takes(g);
    }
}
