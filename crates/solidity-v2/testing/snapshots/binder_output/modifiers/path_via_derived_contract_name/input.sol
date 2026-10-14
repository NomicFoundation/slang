pragma solidity *;

contract A {
    modifier m() {
        _;
    }
}

contract B is A {}

contract C is B {
    // A modifier path through a contract's name also reaches its bases.
    function test() internal B.m {}
}
