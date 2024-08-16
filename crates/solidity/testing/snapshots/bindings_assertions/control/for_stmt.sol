contract Test {
    function simple_loop() public returns (int) {
        int x = 1;
        //  ^def:1
        for (int i = 0; i < 10; i++) {
            //   ^def:2
            //          ^ref:2
            //                  ^ref:2
            x = x * 2;
            //<ref:1
            //  ^ref:1
        }
        return x + i;
        //         ^ref:! (>= 0.5.0)
        //         ^ref:2 (< 0.5.0)
    }

    function loop_with_outside_var() public {
        int z;
        //  ^def:3
        int w = 0;
        //  ^def:4
        for ( z = 10; z > 0; w += z) {
            //^ref:3
            //        ^ref:3
            //               ^ref:4
            //                    ^ref:3
            z--;
            //<ref:3
        }
    }
}
