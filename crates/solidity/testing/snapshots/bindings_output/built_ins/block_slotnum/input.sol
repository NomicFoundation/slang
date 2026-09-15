contract Test {
    function test() public {
        uint64 slot = block.slotnum; // added in 0.8.37
    }
}
