// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[2][] items;

    function g(uint256[] memory value) public {
        // TODO(validation) SDR[1686,1750,1751]: a dynamic array is pushed into a fixed-size one.
        items.push(value);
    }
}
