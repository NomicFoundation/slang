// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata) external {}

    function g(bytes memory b) public {
        this.f(b);
    }

    function h(function(bytes calldata) external fp, bytes memory b) internal {
        fp(b);
    }
}
