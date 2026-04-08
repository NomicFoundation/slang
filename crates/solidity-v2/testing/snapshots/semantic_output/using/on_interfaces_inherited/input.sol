library LibFoo {
    function use_foo(IFoo x) public returns (int) {}
}
interface IFoo {
}
contract Base {
    using LibFoo for IFoo;
}
contract Test is Base {
    function test(address x) public {
        // should work for Solidity < 0.7.0
        IFoo(x).use_foo();
    }
}
