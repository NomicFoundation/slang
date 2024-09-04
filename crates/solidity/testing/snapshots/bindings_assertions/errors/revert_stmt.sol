contract Test {
    error InvalidState();
    //    ^def:1

    function testRevert() public {
        revert InvalidState();
        //     ^ref:1 (>= 0.8.4)
    }

    function testOtherRevert() public {
        int code = 10;
        //  ^def:2
        revert Utils.GenericError(code);
        //                        ^ref:2 (>= 0.8.4)
        //           ^ref:4 (>= 0.8.4)
        //     ^ref:3 (>= 0.8.4)
    }
}

library Utils {
    //  ^def:3
    error GenericError(int code);
    //    ^def:4
}
