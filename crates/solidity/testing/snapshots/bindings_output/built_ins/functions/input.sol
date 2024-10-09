contract BuiltInsTest {
    function testRequire() public {
        require(true, "should always succeed");
    }

    function testRevert() public {
        revert("testing revert");
    }

    function testAssert() public {
        assert(2 + 2 == 4);
    }
}
