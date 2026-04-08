contract AssemblyFunctions {
    function test(uint256 x) public returns (uint256 r) {
        assembly {
            function outer1(a) -> b {
                b := add(a, 1)
            }

            {
                r := add(outer1(x), inner(x))
                function inner(f) -> g {
                    g:= mul(f, outer2(f))
                }
            }

            function outer2(c) -> d {
                d := mul(c, outer2_inner(c))

                function outer2_inner(e) -> f {
                    f := e
                }
            }
        }
    }
}
