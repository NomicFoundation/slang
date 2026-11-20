// SPDX-License-Identifier: MIT
pragma solidity *;

struct S {
    function(S memory) external f;
}

struct T {
    function(T memory) internal f;
}

contract C {
    function missing(T memory t) internal pure returns (bytes4) {
        // An internal function value has no selector.
        return t.f.selector;
    }

    function present(S memory s) internal pure returns (address, bytes4) {
        // A function-typed field is not attached to the struct, so it keeps
        // its external call mode, even though it takes the struct first.
        return (s.f.address, s.f.selector);
    }
}
