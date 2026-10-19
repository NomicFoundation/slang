// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    uint256 constant K = 1;
}

contract C {
    function f() public pure returns (uint256) {
        return D.K;
    }
}
