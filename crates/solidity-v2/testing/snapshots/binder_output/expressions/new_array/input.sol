struct Foo {
    uint x;
}

contract Test {
    function test() internal pure {
        new Foo[](0).length;
    }
}
