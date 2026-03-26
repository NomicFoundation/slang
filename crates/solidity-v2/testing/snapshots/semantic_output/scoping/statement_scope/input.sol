contract Base {
    function foo() public returns (address) {}
}

contract Test is Base {
    function test() public {
        (bool foo, ) = foo().call('');
    }
}
