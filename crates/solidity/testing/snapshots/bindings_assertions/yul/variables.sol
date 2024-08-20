contract AssemblyVariables {
    function yul_let(uint256 x) public returns (uint256 z) {
        //                                              ^def:2
        //                   ^def:1
        assembly {
            let y := 123 + x
            //             ^ref:1
            //  ^def:3
            z := 456 + y
            //         ^ref:3
            //<ref:2
        }
    }
}
