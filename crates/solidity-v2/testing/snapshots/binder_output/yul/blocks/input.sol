contract AssemblyBlocks {
    function yul_let(uint256 x) public returns (uint256 z) {
        assembly {
            let zero := 0
            let v := zero
            {
                let y := x
                z := v
                v := y
            }
            v := zero
        }
    }
}
