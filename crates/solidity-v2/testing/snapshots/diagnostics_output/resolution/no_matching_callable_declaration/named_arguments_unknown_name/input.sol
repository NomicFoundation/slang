// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x, bool y) internal pure {}

    function f(uint256 x, string memory y) internal pure {}

    function no_overload_accepts() internal pure {
        // Neither overload declares a parameter named `z`.
        f({x: 1, z: true});
    }

    function one_overload_accepts() internal pure {
        f({x: 1, y: true});
    }
}
