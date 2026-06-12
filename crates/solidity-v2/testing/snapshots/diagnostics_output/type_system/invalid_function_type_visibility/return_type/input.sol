// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() external returns (function (uint256) private) {}
}
