// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // An external payable function type is allowed.
    function f(function() external payable cb) public {}
}
