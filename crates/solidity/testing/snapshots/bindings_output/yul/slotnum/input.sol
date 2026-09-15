contract Test {
    function test() public {
        assembly {
            let slot := slotnum() // added in 0.8.37
        }
    }
}
