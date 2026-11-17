// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256) external {}
}

contract C {
    using L for uint256;

    function missing(uint256 x) internal {
        x.f{gas: 1000}();
    }
}
