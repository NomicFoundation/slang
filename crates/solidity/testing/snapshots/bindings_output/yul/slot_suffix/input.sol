contract Test {
    function test(bytes storage data) public {
        assembly {
            let s := sload(data_slot)
        }
    }
}
