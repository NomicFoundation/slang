// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    error Err(uint256 a);

    function g(bool c) public {
        require(c, Err({b: 1}));
    }
}
