// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    event E();
}

contract C is A {
    function f() public {
        uint256 x;
        emit E();
        x;
    }
}
