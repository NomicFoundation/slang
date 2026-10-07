// SPDX-License-Identifier: MIT
pragma solidity *;

// The literal is typed `address`, but a length is folded from its digits.
contract C {
    uint256[0x0000000000000000000000000000000000000003] arr;
}
