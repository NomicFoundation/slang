library Utils {
    function foo(address payable _to) internal {}
}

contract Test {
    using Utils for address payable;

    function test(address _to) internal {
        address(uint160(_to)).foo();  // should bind in >= 0.5.0 and < 0.8.0
    }
}
