// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[2][] items;

    function g(uint256[3] memory value) public {
        // TODO(validation) SDR[1686,1750,1751]: a fixed-size array is pushed into a shorter one.
        items.push(value);
    }
}
