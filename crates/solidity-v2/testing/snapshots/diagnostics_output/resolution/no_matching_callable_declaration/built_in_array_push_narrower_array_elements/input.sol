// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint8[2][] items;

    function g(uint16[2] memory value) public {
        // TODO(validation) SDR[1686,1750,1751]: the elements do not convert to the narrower type.
        items.push(value);
    }
}
