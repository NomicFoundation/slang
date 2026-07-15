// SPDX-License-Identifier: MIT
pragma solidity *;

function foo(uint256 self) pure returns (uint256) {
    return self;
}

uint256 constant foo = 1;

contract C {
    using {foo} for uint256;
}
