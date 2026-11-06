// SPDX-License-Identifier: MIT
pragma solidity *;

// A tuple has a mobile type only if every component does. This is reported
// before the tuple is rejected for not being a valid array element.

library L {
    function lf(uint256) internal pure {}
}

contract C {
    using L for uint256;

    function f(uint256 x) public pure {
        [(x.lf, 1)];
    }
}
