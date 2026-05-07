interface IFoo {
    function foo(address, uint) external;
}

library LibFoo {
    function foo(IFoo, uint) external {}
}

contract Test {
    using LibFoo for IFoo;

    function test(IFoo f) internal {
        f.foo(1); // invokes the attached function
        f.foo(msg.sender, 1); // invokes the interface function
    }
}
