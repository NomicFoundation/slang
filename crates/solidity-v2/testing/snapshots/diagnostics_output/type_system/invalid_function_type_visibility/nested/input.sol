// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() external pure returns (function () internal returns (function () public)) {}
}
