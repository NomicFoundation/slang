// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint16[2][] items;

    function g(uint8[2] memory value) public {
        items.push(value);
    }
}
