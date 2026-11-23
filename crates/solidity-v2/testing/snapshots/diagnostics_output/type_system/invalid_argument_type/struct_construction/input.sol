// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct T {
        uint256 x;
    }

    struct S {
        T t;
        uint256[] xs;
        string s;
        bool b;
    }

    T st;
    uint256[] sxs;

    function g(string calldata cs) public view {
        S(st, sxs, cs, true);
        S(T(1), new uint256[](1), "x", 1);
    }
}
