// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: the outer key `owner` conflicts with the inner value `owner`,
    // across the nested mapping.
    mapping(address owner => mapping(address hello => address owner)) m;
}
