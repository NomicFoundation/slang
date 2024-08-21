contract AssemblyFunctions {
    function test(uint256 x) public returns (uint256 r) {
        assembly {
            function outer1(a) -> b {
                //   ^def:1
                b := add(a, 1)
            }

            {
                r := add(outer1(x), inner(x))
                //                  ^ref:2
                //       ^ref:1
                function inner(f) -> g {
                    //   ^def:2
                    g:= mul(f, outer2(f))
                    //         ^ref:3
                }
            }

            function outer2(c) -> d {
                //   ^def:3
                d := mul(c, outer2_inner(c))
                //          ^ref:4

                function outer2_inner(e) -> f {
                    //   ^def:4
                    f := e
                }
            }
        }
    }
}
