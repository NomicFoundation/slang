contract AssemblyVariable {
    function yul_let() public returns (uint256 z) {
        assembly {
            let x := 123
            z := 456
        }
    }
}
