// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x, string memory y, bool z) internal pure {}

    function f(string memory y, uint256 x, bool z) internal pure {}

    function g(uint256 x, bool z) internal pure {}

    function ambiguous() internal pure {
        // Both overloads declare the same parameter names, so naming the
        // arguments doesn't tell them apart.
        f({x: 1, y: "abc", z: true});
    }

    function unambiguous() internal pure {
        // `g` is not overloaded.
        g({x: 1, z: true});
    }
}
