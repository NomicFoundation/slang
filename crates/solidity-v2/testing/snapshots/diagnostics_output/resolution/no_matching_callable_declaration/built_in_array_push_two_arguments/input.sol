// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[] items;

    function g() public {
        items.push(1, 2);
    }
}
