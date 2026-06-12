// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function (uint256) internal x;
    function (uint256) external y;
    function (uint256) z;

    function f(function (uint256) external g) external pure {}
}
