contract Base {
    modifier foo { _; }
}

contract Test is Base {
    function testUnqualified() public foo {
    }
    function testQualified() public Base.foo {
    }
}
