contract Test {
    uint256 add;

    function test() public view returns (uint256 result) {
        assembly {
            // `add` here resolves to the state variable, not the built-in
            result := sload(add.slot)
        }
    }
}
