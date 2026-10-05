// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function(uint256) external payable returns (uint256) x;

    function missing() public {
        x.value(2)(1);              // should be rejected
    }

    function present() public {
        x{value: 2}(1);              // should be accepted
    }
}
