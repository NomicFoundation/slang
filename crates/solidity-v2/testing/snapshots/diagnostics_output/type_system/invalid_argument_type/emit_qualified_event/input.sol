// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    event E(uint256 a);
}

contract C is B {
    function g() public {
        emit B.E(true);
    }
}
