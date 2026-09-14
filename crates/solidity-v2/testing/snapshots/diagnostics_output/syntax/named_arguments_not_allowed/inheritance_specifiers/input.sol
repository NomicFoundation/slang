// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    constructor(uint256 value) {}
}

// Valid: an inheritance specifier accepts positional arguments.
contract Positional is Base(1) {}

// Named arguments are not allowed in an inheritance specifier.
contract Named is Base({value: 2}) {}
