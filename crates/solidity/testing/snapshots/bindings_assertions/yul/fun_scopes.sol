contract AssemblyFunctions {
    function test(uint256 a) public returns (uint256 r) {
        //                                           ^def:4
        //                ^def:3
        assembly {
            {
                function inner(x_i) -> x {
                    //   ^def:1
                    x := add(x_i, outer())
                    //            ^ref:2
                }

                r := inner(a)
                //         ^ref:3
                //   ^ref:1
                //<ref:4
            }

            let z := 1
            //  ^def:5

            function outer() -> y {
                //   ^def:2
                y := inner()
                //   ^ref:!  -- function is not visible here
                r := add(a, 1)
                //       ^ref:!  -- assembly functions cannot access Solidity variables
                //<ref:!
                y := z
                //   ^ref:! -- vars declared outside the function are not visible either
            }
        }
    }
}
