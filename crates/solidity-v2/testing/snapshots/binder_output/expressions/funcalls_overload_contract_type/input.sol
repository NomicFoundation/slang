interface Foo {
    function add(address) external returns (int);
    function add(int, int) external returns (int);
}

contract Bar {
    function someFunc(Foo other, address x) internal {
        other.add(x);
        // `this` is not implicitly convertible to `address`
        other.add(this);
    }
}
