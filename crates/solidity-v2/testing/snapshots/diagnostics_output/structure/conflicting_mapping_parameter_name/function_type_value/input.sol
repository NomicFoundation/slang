// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The value type is a function type whose parameter is itself a mapping.
    // That inner mapping is a separate namespace: its `a => a` reuse is flagged,
    // while the outer mapping (key `k`, function-type value) has no conflict.
    mapping(uint k => function(mapping(uint a => uint a) storage) internal) m;
}
