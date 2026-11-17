// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(uint256) internal pure {}

    function f() public pure {
        // Invalid: a call argument is a read position. Only the omitted
        // component is reported, not the argument's type.
        g((1, ));
    }
}
