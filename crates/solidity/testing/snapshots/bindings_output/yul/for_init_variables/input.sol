contract Test {
    function test() public {
        for (uint256 i = 0; i < 10; i += 1) {
            assembly {
                let x := add(i, 1)
            }
        }
    }
}
