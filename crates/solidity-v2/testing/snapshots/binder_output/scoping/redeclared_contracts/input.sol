contract A {
    function f() public {}
}

contract A {
    function g() public {}
}

contract Test {
    A a;

    function test() public {
        a.f();
        a.g();
        new A();
    }
}
