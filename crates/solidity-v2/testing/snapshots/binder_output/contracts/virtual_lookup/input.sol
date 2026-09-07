pragma solidity *;

contract Base {
    function foo() public virtual {}
    function bar() public {
        foo();
    }
}

contract Derived is Base {
    function foo() public override {}
}
