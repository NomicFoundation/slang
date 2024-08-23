contract A {
    //   ^def:dummy    -- to have at least one assertion in 0.4.11
    function foo() public pure virtual returns (string memory) {
        //   ^def:1
        return "A";
    }
}

contract B is A {
    function foo() public pure virtual override returns (string memory) {
        //   ^def:2
        return "B";
    }
}

contract C is A {
    function foo() public pure virtual override returns (string memory) {
        //   ^def:3
        return "C";
    }
}

contract D is B, C {
    function foo() public pure override(B, C) returns (string memory) {
        return super.foo();
        //           ^ref:3
    }
}

contract E is C, B {
    function foo() public pure override(C, B) returns (string memory) {
        return super.foo();
        //           ^ref:2
    }
}
