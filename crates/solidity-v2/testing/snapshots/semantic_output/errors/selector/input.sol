contract Test {
    error TestError();

    function test() public {
        TestError.selector;
    }
}
