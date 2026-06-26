contract Test {
    function test(uint256 add) public pure returns (uint256 result) {
        assembly {
            // `add` is a Yul built-in, so it shadows the Solidity parameter
            // `add` in both call position (the outer `add`) and value position
            // (the inner `add`). The parameter is never referenced.
            result := add(1, add)
        }
    }
}
