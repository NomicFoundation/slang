// SPDX-License-Identifier: MIT
pragma solidity *;

// A public library function takes a storage parameter by reference, so a
// memory argument matches neither overload.
library L {
    function foo(bool a, uint256[] storage x) public {}

    function foo(bool a, string memory x) public {}
}

contract C {
    using L for bool;

    function test(uint256[] memory x) internal {
        false.foo(x);
    }
}
