// SPDX-License-Identifier: MIT
pragma solidity *;

type Int is int16;

using {add as +} for Int global;

function add(Int, Int) pure returns (Int) {}

function f() pure {
    Int a;
    // `add` is only attached as the `+` operator, not as a member.
    a.add(a);
}
