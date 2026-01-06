contract Test {
    function testBuiltIn() public pure {
        revert("error");
    }

    function testShadowed() public pure {
        function(string memory) pure revert = customRevert;
        revert("error");
    }

    function customRevert(string memory) internal pure {}
}
