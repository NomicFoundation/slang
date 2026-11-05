// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    event E();
}

contract C {
    function f() public pure returns (bytes32) {
        return D.E.selector;
    }
}
