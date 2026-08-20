// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {}

    function f(address) public pure {}

    function g() public pure {}

    function ambiguous() public pure {
        assembly {
            // Yul does no overload resolution, so `f` can't be narrowed down.
            let x := f
        }
    }

    function unambiguous() public pure {
        assembly {
            let x := g
        }
    }
}
