contract YulStackAssignment {
    function test() public {
        assembly {
            let x := 0
            sload(10)
            =: x
        }
    }
}
