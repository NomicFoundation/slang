// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        selfdestruct(address(0));
    }
}
