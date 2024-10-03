contract Base {
    modifier foo { _; }
}

contract Test is Base {
    function test() public foo {
    }
}
