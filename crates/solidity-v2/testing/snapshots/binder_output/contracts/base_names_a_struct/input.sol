pragma solidity *;

// A base must be a contract or an interface. Naming a struct is reported, but
// the reference still points at the struct.
struct S {
    uint256 a;
}

contract C is S {}
