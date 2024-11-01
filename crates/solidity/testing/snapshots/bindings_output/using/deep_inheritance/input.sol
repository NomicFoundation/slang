interface IFoo {}
contract Vars {
    IFoo internal constant foo = IFoo(address(0x0));
}
library LibFoo {
    function use_foo(IFoo f) internal {}
}
contract Base is Vars {
}
contract Test is Base {
    using LibFoo for IFoo;
    function test() public {
        foo.use_foo();
    }
}
