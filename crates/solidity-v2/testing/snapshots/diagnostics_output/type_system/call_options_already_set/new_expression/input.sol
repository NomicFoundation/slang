// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    constructor() payable {}
}

contract C {
    function g() public {
        new D{value: 1}{salt: bytes32(0)}();
    }
}
