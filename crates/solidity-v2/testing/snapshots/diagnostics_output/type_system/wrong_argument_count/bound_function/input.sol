// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256, uint256) internal {}
}

contract C {
    using L for uint256;

    function g(uint256 x) public {
        x.f(1);
        x.f(1, 2);
    }
}
