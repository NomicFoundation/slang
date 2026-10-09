pragma solidity *;

contract A {
    error E();
}

contract B is A {}

contract C is B {
    function inherited() internal pure {
        // A contract's type name only exposes the errors it declares.
        revert B.E();
    }

    function declared() internal pure {
        revert A.E();
    }
}
