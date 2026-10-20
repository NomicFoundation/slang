// SPDX-License-Identifier: MIT
pragma solidity *;

// A `uint8[]` argument is not a `uint256[]`, so only one overload accepts the
// external call and it is not ambiguous.
contract C {
    function f(uint256[] memory a) external pure returns (uint256) {
        return a.length;
    }

    function f(uint8[] memory a) external pure returns (uint256) {
        return a.length + 1;
    }

    function g(uint8[] memory a) external view returns (uint256) {
        return this.f(a);
    }
}
