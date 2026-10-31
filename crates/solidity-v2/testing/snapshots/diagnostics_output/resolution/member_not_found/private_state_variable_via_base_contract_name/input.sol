// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    uint256 private x;
}

contract D is B {
    function g() public view returns (uint256) {
        // A private member is not reachable through the contract name.
        return B.x;
    }
}
