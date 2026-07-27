// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: the key and value parameters share the name `owner`.
    mapping(address owner => address owner) m;
}
