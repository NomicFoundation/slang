// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A function type with no explicit visibility defaults to internal,
    // so a payable one is still rejected.
    function f(function() payable cb) public {}
}
