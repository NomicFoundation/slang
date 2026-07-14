// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Non-payable function types are allowed regardless of visibility.
    function f(function() internal cb, function() external cb2) internal {}
}
