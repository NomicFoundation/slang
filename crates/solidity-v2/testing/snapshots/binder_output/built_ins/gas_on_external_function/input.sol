// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function(uint256) external returns (uint256) x;

    function missing() public {
        x.gas(2)(1);              // should be rejected
    }

    function present() public {
        x{gas: 2}(1);              // should be accepted
    }
}
