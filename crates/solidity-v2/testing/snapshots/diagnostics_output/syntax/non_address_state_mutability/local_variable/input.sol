// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // A state-mutability specifier (`payable`) may only follow `address`,
        // not another elementary type.
        uint256 payable x;
    }
}
