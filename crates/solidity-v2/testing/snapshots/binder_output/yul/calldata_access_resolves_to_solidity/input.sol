contract Test {
    function test(bytes calldata add) public pure returns (uint256 len, uint256 off) {
        assembly {
            // `add` below resolves to the calldata parameter, not the built-in
            len := add.length
            off := add.offset
        }
    }
}
