// SPDX-License-Identifier: MIT
pragma solidity *;

// The local `A` shadows the contract, and has no member `E`.
contract A {
    event E();
}

contract C {
    function f() public {
        uint256 A;
        emit A.E();
    }
}
