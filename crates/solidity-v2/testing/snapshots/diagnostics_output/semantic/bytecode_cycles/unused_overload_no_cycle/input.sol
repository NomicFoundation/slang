// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the string overload is ever called, so the uint256 overload's `new A`
// is unreachable and there is no cycle.

contract A {
    uint256 counter;

    function f(uint256) internal {
        new A();
    }

    function f(string memory) internal {
        counter = 1;
    }

    function g() public {
        f("");
    }
}
