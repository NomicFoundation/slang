pragma solidity *;

contract A {
    struct S {
        uint256 x;
    }
}

contract B is A {}

contract C is B {
    function test() internal pure {
        // A type path through a contract's name also reaches its bases.
        B.S memory s;
        s;
    }
}
