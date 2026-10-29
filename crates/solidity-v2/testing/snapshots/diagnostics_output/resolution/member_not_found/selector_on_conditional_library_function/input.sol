// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external {}
}

contract C {
    function e() external {}

    function missing(bool b) internal pure returns (bytes4) {
        // The conditional no longer names the declaration.
        return (b ? L.f : L.f).selector;
    }

    function present(bool b) internal view returns (bytes4, address) {
        return ((b ? this.e : this.e).selector, (b ? this.e : this.e).address);
    }
}
