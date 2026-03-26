contract Foo {
    int public x;
}

contract Bar {
    int public y;
    Foo f;

    function test() public returns (int) {
        return y + this.y() + f.x();
    }
}
