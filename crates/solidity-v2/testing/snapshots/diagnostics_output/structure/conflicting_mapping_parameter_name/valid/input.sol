// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Valid: every mapping parameter name is distinct.
    mapping(address key => mapping(address inner => address value)) m;
}
