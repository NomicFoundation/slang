// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {
        try this.f() {} catch (X x) {}
    }
}
