// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 a, uint256 b) public pure returns (uint256) {
        return a + b;
    }

    // Invalid: the named argument `a` is provided twice.
    function callWithDuplicate() public pure returns (uint256) {
        return f({a: 1, a: 2});
    }

    // Valid: each named argument is distinct.
    function callValid() public pure returns (uint256) {
        return f({a: 1, b: 2});
    }
}
