function add(uint256 a, uint256 b) pure returns (uint256) {
    return a + b;
}

contract Test {
    function test() public pure {
        assembly {
            // `add` is a Yul built-in, so it shadows the Solidity free
            // function `add` even though that function is in scope.
            let result := add(1, 2)
        }
    }
}
