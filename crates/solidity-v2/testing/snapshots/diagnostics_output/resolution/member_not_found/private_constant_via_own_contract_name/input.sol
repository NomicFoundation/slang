// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 private constant K = 1;

    function g() public pure returns (uint256) {
        // A private member is not reachable through the contract name, not
        // even from within its own contract.
        return C.K;
    }
}
