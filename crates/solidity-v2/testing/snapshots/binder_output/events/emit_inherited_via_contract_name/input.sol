pragma solidity *;

contract A {
    event E();
}

contract B is A {}

contract C is B {
    function test() internal {
        // A contract's type name only exposes the events it declares.
        emit B.E();
        emit A.E();
    }
}
