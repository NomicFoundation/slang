// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct S {
        mapping(uint256 => uint256) values;
    }

    S[] items;

    function g() public {
        // TODO(validation) SDR[1615]: an element holding a mapping cannot be pushed.
        items.push(items[0]);
    }
}
