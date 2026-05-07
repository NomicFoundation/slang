contract Test {
    type Internal is uint;

    function test() public returns (uint) {
        Internal inter = Internal.wrap(10);
        return Internal.unwrap(inter);
    }
}
