// SPDX-License-Identifier: MIT
pragma solidity *;

// Attaching ignores data locations but not element types: a `uint8` array is
// not a `uint256` array, even though `uint8` widens to `uint256`.
library L {
    function g(uint256[] memory c) internal pure returns (uint256) {
        return c.length;
    }
}

contract C {
    using L for uint8[];

    uint8[] a;

    function test() internal view returns (uint256) {
        return a.g();
    }
}
