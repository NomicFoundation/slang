// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    function f(bytes calldata) public {}
    function f(uint256) public {}
}

contract C is B {
    function internal_call(bytes memory b) public {
        super.f(b);
    }
}
