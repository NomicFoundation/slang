// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract C {
    struct Inner {
        uint256 c;
    }

    struct Outer {
        Inner b;
    }

    Outer a;

    // The periods of a member access chain are recovered from the source, so any
    // trivia in between (which can contain periods of its own) has to be skipped.
    function f() public view returns (uint256) {
        return a /* .x */ . /* .y */ b // .z
            .c;
    }
}
