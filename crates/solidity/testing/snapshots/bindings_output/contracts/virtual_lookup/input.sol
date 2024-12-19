contract Base {
    function foo() public {}
    function bar() public {
        foo();
    }
}

contract Derived is Base {
    function foo() public {}
}
