// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    constructor(uint256 a) payable {}
}

contract D {
    function g() public {
        new C{value: 1}(true);
    }
}
