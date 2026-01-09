contract Test {
    function testBuiltIn() public {
        revert("error");
    }

    function testBuiltInInCondition() public {
        if (revert("error")) {}
    }

    function testRevertCanBeShadowed() public {
        function(string memory) revert = customRevert;
        revert("error");
    }

    function customRevert(string memory) internal {}
}
