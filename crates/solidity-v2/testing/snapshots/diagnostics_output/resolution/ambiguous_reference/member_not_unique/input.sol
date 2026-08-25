// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function a(uint256) public returns (uint) {
        return 1;
    }

    function a(uint8) public returns (uint) {
        return 2;
    }

    function b(uint8) public returns (uint) {
        return 3;
    }

    function self() public returns (C) {
        return this;
    }

    function ambiguous() internal returns (function(uint8) external returns (uint)) {
        // The overload set is never called, so nothing narrows it down.
        return self().a;
    }

    function unambiguous() internal returns (function(uint8) external returns (uint)) {
        return self().b;
    }
}
