contract Lock {}

contract Test {
    function test() public {
        Lock l = (new Lock).value(1)();
    }
}
