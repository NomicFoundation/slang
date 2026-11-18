// SPDX-License-Identifier: MIT
pragma solidity *;

// Every item of an array literal needs a mobile type, and a function with a
// bound first argument has none. Each item is reported, not only the first.

library L {
    function lf(uint256) internal pure {}
    function lg(uint256) internal pure {}
}

contract C {
    using L for uint256;

    function f(uint256 x) public pure {
        [x.lf, x.lg];
    }
}
