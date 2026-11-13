// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(bytes calldata a) public {}

    function g(bytes memory b) internal {
        f({a: b});
    }
}

contract C {
    function h(bytes memory b) public {
        L.f({a: b});
    }
}
