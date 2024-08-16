contract Test {
    function empty_init() public {
        int i = 1;
        //  ^def:1
        int x = 0;
        //  ^def:2
        for (; i < 10; i++) {
            //         ^ref:1
            // ^ref:1
            x += i;
            //<ref:2
            //   ^ref:1
        }
    }

    function empty_condition() public {
        for (int i = 0;; i++) {
            //   ^def:3
            //           ^ref:3
            if (i > 10) break;
            //  ^ref:3
        }
    }
}
