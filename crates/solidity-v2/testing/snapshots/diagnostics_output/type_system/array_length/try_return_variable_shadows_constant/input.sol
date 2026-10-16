// SPDX-License-Identifier: MIT
pragma solidity *;

// The try return variable `N` shadows the constant, so the length is not a constant.
contract C {
    uint256 constant N = 2;

    function g() external pure returns (uint256) {
        return 1;
    }

    function f() public view {
        try this.g() returns (uint256 N) {
            uint256[N] memory arr;
        } catch {}
    }
}
