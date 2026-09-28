// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() internal view returns (address) {
        // Inside a library, `this` converts to its address.
        return address(this);
    }
}
