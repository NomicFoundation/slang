pragma solidity *;

type Int is int16;

using {add as +, add} for Int global;

function add(Int, Int) pure returns (Int) {}

function f() pure {
    Int a;
    a.add(a);
    a + a;
}
