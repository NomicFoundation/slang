// SPDX-License-Identifier: MIT
pragma solidity *;

// Mappings are omitted, leaving nothing to return.
contract C {
    struct S {
        mapping(uint256 => uint256) m;
    }

    S public s;
}
