pragma solidity *;

// An internal member is not reachable through the name of a contract outside
// the hierarchy. That is reported, but the reference still points at it.
contract A {
    uint256 internal x;
}

contract B {
    function f() public view {
        A.x;
    }
}
