// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(bytes calldata) public {}

    function g(bytes memory b) internal {
        f(b);
    }
}

contract C {
    function h(bytes memory b) public {
        L.f(b);
    }
}
