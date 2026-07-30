// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: `owner` is reused several times across the nested mapping; each
    // reuse is reported at its own location.
    mapping(address owner => mapping(address owner => address owner)) m;
}
