contract Test {
    function test() public {
        //   ^def:dummy
        assembly {
            let n := calldataload(4)
            let a := 1
            let b := a
        loop:
        //<def:1 (< 0.5.0)
            jumpi(loopend, eq(n, 0))
            //    ^ref:2 (< 0.5.0)
            //    ^ref:! (>= 0.5.0)
            a add swap1
            n := sub(n, 1)
            jump(loop)
            //   ^ref:1 (< 0.5.0)
            //   ^ref:! (>= 0.5.0)
        loopend:
        //<def:2 (< 0.5.0)
            mstore(0, a)
            return(0, 0x20)
        }
    }
}
