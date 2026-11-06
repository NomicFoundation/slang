// SPDX-License-Identifier: MIT
pragma solidity *;

// An error's parameters are not visible to the types of its parameters, so
// `StructType` still names the struct.
contract C {
    struct StructType {
        uint256 x;
    }

    error E(StructType StructType);
}
