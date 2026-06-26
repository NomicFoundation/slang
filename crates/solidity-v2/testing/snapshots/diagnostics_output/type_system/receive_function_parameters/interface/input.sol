// SPDX-License-Identifier: MIT
pragma solidity *;

// The same rule applies to a receive declared inside an interface.
interface I {
    receive(uint256) external payable;
}
