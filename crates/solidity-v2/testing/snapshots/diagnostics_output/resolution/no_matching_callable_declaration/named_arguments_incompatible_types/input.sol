// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x, bool y) internal pure {}

    function f(uint256 x, string memory y) internal pure {}

    function no_overload_accepts() internal pure {
        // Both parameter names match, but neither overload takes an address.
        f({x: 1, y: address(0)});
    }

    function one_overload_accepts() internal pure {
        f({x: 1, y: true});
    }
}
