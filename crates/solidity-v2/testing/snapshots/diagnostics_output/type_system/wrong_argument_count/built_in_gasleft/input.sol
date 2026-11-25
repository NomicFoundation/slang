// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        gasleft(1);
    }
}
