pragma solidity *;

// Before 0.8.21, an event is not reachable through the name of a contract
// outside the hierarchy. That is reported, but the reference still points at it.
contract A {
    event E();
}

contract B {
    function f() public {
        emit A.E();
    }
}
