// --- context: Derived
contract Base {
    function foo() public {}
    //       ^def:1
    function bar() public {
        foo();
        //<ref:2
    }
}

contract Derived is Base {
    function foo() public {}
    //       ^def:2
}
