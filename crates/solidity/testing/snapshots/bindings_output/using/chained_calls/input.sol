library Foo {
    struct Bar {
        uint value;
    }
    function noop(uint x) public returns (uint) {}
    function bar(uint x) public returns (Bar memory) {}
}

contract Test {
    using Foo for uint;
    function test(uint a, Foo.Bar memory b) public {
        uint[] memory xs;
        a.noop().noop().noop();
        b.value.noop().bar().value.noop();
        xs[5].noop().noop();
    }
}
