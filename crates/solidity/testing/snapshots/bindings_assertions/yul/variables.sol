contract AssemblyVariables {
    function yul_let(uint256 x) public returns (uint256 z) {
        //                                              ^def:2
        //                   ^def:1
        assembly {
            let y := add(123, x)
            //                ^ref:1
            //  ^def:3
            z := add(456, y)
            //            ^ref:3
            //<ref:2
            let w := add(w, 1)
            //           ^ref:!  -- vars are not accessible in the right side of their decl
            //  ^def:4
        }
    }
}
