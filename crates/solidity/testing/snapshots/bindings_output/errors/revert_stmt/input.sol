contract Test {
    error InvalidState();

    function testRevert() public {
        revert InvalidState();
    }

    function testOtherRevert() public {
        int code = 10;
        revert Utils.GenericError(code);
    }

    function revertFunction() public {
        revert("error");
    }


    function revertFunctionInCondition() public {
        if (revert("error")) {}
    }

}

library Utils {
    error GenericError(int code);
}
