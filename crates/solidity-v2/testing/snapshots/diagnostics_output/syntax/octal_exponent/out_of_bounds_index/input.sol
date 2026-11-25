// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[100000001] a; // 1e8 + 1

    function f() public view returns (uint256) {
        // solc reads the exponent as octal: `1e010` is `1e8`, within bounds,
        // while `1e10` is well past it.
        return a[1e010] + a[1e10];
    }
}
