// SPDX-License-Identifier: MIT
pragma solidity *;

// The same rule applies to a receive declared inside an interface.
interface I {
    receive(uint256) external payable;
}

// With multiple parameters we should diagnose over all of them
interface J {
    receive(uint256, bool) external payable;
}
