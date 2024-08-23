contract YulStackAssignment {
    function test() public {
        assembly {
            let x := 0
            //  ^def:1
            sload(10)
            =: x
            // ^ref:1 (< 0.5.0)
        }
    }
}
