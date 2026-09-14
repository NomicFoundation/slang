// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    constructor(uint256 value) {}
}

// Valid: a base constructor call accepts positional arguments.
contract Positional is Base {
    constructor() Base(1) {}
}

// Named arguments are not allowed in a base constructor call.
contract Named is Base {
    constructor() Base({value: 2}) {}
}
