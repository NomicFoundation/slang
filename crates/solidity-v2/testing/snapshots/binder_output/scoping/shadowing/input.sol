function foo() {}
function bar() {}

contract Base {
    function bar() public {}
}

contract Test is Base {
    int x;

    function test(int x) public {
        foo();
        bar();
        x;
    }

    function foo() internal {}
}
