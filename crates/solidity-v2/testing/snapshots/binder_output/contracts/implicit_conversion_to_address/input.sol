library FooUtils {
    function nop(address) internal {}
}

contract Foo {}

contract Test {
    using FooUtils for Foo;

    function test(Foo f) internal {
        f.nop(); // in Solidity <0.5.0 contract references are convertible to address
    }
}
