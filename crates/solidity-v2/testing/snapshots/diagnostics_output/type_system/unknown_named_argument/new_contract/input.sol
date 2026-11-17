// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    constructor(uint256 a) {}
}

contract D {
    function g() public {
        new C({a: 1});
        new C({b: 1});
    }
}
