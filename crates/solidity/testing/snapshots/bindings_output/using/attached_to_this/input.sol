library Utils {
    function foo(ITest subject) internal {}
}

interface ITest {}

contract Test is ITest {
    using Utils for *;

    function bar() internal {}

    function test() internal {
        this.foo();
    }
}
