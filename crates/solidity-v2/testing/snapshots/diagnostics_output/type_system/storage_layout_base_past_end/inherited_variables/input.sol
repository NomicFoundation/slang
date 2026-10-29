// SPDX-License-Identifier: MIT
pragma solidity *;

// From slot 1, `B`'s array alone fits, but `C` lays out its own variable
// after it, which reaches `2**256`.
contract B {
    uint256[2 ** 256 - 2] a;
}

contract C is B layout at 1 {
    uint256 b;
}
