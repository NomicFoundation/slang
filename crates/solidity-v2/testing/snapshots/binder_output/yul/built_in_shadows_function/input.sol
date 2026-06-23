function add(uint256 a, uint256 b) pure returns (uint256) {
    return a + b;
}

contract Test {
    function test() public pure {
        assembly {
            let x := add(1, 2)
        }
    }
}
