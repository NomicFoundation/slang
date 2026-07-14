// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // An explicitly `internal` payable function type is rejected.
    function f(function() internal payable cb) public {}
}
