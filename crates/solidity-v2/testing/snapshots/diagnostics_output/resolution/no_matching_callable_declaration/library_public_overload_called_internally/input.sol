// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(bytes calldata) public {}
    function f(uint256) public {}

    function internal_call(bytes memory b) public {
        f(b);
    }
}

contract C {
    function delegate_call(bytes memory b) public {
        L.f(b);
    }
}
