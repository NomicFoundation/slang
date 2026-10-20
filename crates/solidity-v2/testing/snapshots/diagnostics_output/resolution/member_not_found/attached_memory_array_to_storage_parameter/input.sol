// SPDX-License-Identifier: MIT
pragma solidity *;

// A storage parameter is a reference into storage, so a memory receiver does
// not attach to it.
library L {
    function g(uint256[] storage c) internal view returns (uint256) {
        return c.length;
    }
}

contract C {
    using L for uint256[];

    function test(uint256[] memory a) internal view returns (uint256) {
        return a.g();
    }
}
