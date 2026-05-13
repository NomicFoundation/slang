interface IFoo {}
contract Bar {}
contract Foo is IFoo, Bar {}

library Lib {
    function useIFoo(IFoo) internal {}
    function useIFoo(uint) internal {}
    function useBar(Bar) internal {}
    function useBar(uint) internal {}
}

contract Test {
    using Lib for Foo;

    function test() internal {
        Foo foo;
        foo.useIFoo();
        foo.useBar();
        Lib.useIFoo(foo);
        Lib.useBar(foo);
    }
}
