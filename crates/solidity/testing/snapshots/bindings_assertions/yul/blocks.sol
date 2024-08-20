contract AssemblyBlocks {
    function yul_let(uint256 x) public returns (uint256 z) {
        //                                              ^def:2
        //                   ^def:1
        assembly {
            let zero := 0
            //  ^def:3
            let v := zero
            //       ^ref:3
            //  ^def:4
            {
                z := y
                //   ^ref:!  -- variable is not defined yet
                //<ref:2
                let y := x
                //       ^ref:1
                //  ^def:5
                z := v
                //   ^ref:4
                //<ref:2
                v := y
                //   ^ref:5
                //<ref:4
            }
            v := zero
            //   ^ref:3
            //<ref:4
            z := y
            //   ^ref:!  -- inner block variable is not accessible
            //<ref:2
        }
    }
}
