contract Test {
    function test() public {
        assembly {
            let n := calldataload(4)
            let a := 1
            let b := a
        loop:
            jumpi(loopend, eq(n, 0))
            n := sub(n, 1)
            jump(loop)
        loopend:
            mstore(0, a)
            return(0, 0x20)
        }
    }
}
