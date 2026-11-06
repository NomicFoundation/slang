// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        revert("a", "b");
    }
}
