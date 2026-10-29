// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256) external {}
}

contract C {
    using L for uint256;

    function missing(uint256 x) internal pure returns (address) {
        // A bound library call has no address of its own either.
        return x.f.address;
    }

    function present(uint256 x) internal pure returns (bytes4) {
        return x.f.selector;
    }
}
