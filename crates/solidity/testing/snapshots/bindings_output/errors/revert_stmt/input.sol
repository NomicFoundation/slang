contract Test {
    error InvalidState();

    function testRevert() public {
        revert InvalidState();
    }

    function testOtherRevert() public {
        int code = 10;
        revert Utils.GenericError(code);
    }
}

library Utils {
    error GenericError(int code);
}
