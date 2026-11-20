// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata) public {}
    function f(uint256) public {}

    function external_call(bytes memory b) public {
        this.f(b);
    }

    function internal_call(bytes memory b) public {
        f(b);
    }
}
