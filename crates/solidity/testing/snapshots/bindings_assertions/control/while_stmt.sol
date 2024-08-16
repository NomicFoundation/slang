contract Test {
    function test() public returns (int) {
        int i = 1;
        //  ^def:1
        while (i < 100) {
            // ^ref:1
            int odd = i % 2;
            //  ^def:2
            //        ^ref:1
            i *= 3;
            //<ref:1
        }
        return odd;
        //     ^ref:! (>= 0.5.0)
        //     ^ref:2 (< 0.5.0)
    }
}
