// SPDX-License-Identifier: MIT
pragma solidity *;

// A bound function mixed with a plain function pointer: only the bound item has
// no mobile type.

library L {
    function f(uint256 a) internal pure {}
}

contract C {
    using L for *;

    function f() public pure {
        uint256 t;
        function() pure x;
        [t.f, x][0]({a: 8});
    }
}
