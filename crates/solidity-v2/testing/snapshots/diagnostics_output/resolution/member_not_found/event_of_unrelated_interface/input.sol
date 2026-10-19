// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    event E();
}

contract C {
    function f() public pure returns (bytes32) {
        return I.E.selector;
    }
}
