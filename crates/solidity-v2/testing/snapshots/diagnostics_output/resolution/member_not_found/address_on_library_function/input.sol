// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external {}
}

contract C {
    function missing() internal pure returns (address) {
        // A library call has no address of its own.
        return L.f.address;
    }

    function present() internal pure returns (bytes4) {
        return L.f.selector;
    }
}
