// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 self, uint256 b) internal {}
}

contract C {
    using L for uint256;

    function g(uint256 x) public {
        x.f({b: 1});
        x.f({self: 1});
    }
}
