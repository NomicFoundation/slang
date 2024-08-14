contract Test {
    function test() {
        int x = 1;
        //  ^def:1
        int y = 2;
        //  ^def:2
        if (x > 1) {
            //<ref:1
            int z = 3;
            //  ^def:3
            y = x + 10;
            //  ^ref:1
            //<ref:2
        } else {
            int w = 4;
            //  ^def:4
            y = x + 20;
            //  ^ref:1
            //<ref:2
        }
        return y + z + w;
        //             ^ref:! (>= 0.5.0)
        //             ^ref:4 (< 0.5.0)
        //         ^ref:! (>= 0.5.0)
        //         ^ref:3 (< 0.5.0)
        //     ^ref:2
    }
}
