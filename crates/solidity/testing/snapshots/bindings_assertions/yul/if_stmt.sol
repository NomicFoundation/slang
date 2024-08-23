contract Test {
    function test() public returns (uint256 r) {
        //                                  ^def:1
        uint256 x = 5;
        //      ^def:2
        assembly {
            if gt(x, 2) {
                //^ref:2
                r := foo()
                //   ^ref:3
                //<ref:1
            }
            function foo() -> z {
                //   ^def:3
                z := 1
            }
        }
    }
}
