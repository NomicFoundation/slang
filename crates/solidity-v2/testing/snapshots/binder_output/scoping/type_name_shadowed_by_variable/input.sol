pragma solidity *;

// The nearest declaration wins whatever its kind, so `S` in the type position
// names the variable. That is reported, but the reference still points at it.
struct S {
    uint256 a;
}

contract C {
    function f() public pure {
        uint256 S;
        S memory s;
    }
}
