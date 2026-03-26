contract Test {
    bytes data;
    function test() public {
        assembly {
            let s := sload(data_slot)
            let o := sload(data_offset)
        }
    }
}
