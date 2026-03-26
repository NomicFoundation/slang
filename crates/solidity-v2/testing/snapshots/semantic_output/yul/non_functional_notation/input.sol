contract Test {
    function test() internal {
        // this is valid in Solidity < 0.5.0 only
        assembly {
            0x10 dup1 swap2
            pop pop
        }
    }
}
