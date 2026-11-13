// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() external payable {}

    function g() public view {
        this.f{value: 1}{gas: 2};
    }
}
