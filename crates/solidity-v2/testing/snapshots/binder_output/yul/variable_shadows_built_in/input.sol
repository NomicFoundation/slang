contract Test {
    function test(uint256 add) public pure returns (uint256 result) {
        assembly {
            result := add
        }
    }
}
