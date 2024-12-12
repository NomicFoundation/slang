contract Test {
    bytes data;
    function test() public {
        assembly {
            let s := sload(data.slot)
            let o := sload(data.offset)
        }
    }
}
