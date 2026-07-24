// SPDX-License-Identifier: MIT
pragma solidity *;

// A function attached with `using for` executes within the caller's
// bytecode when it is an internal library function.

library L {
    function g(uint256) internal {
        new C();
    }
}

contract C {
    using L for uint256;

    function h() public {
        uint256 x;
        x.g();
    }
}
