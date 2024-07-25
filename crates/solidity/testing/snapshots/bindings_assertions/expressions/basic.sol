contract Test {
    function playground() returns (int) {
        int x = 1;
        //  ^def:1
        int y = x + 2;
        //      ^ref:1
        //  ^def:2
        return (x + y * x) / y;
        //                   ^ref:2
        //              ^ref:1
        //          ^ref:2
        //      ^ref:1
    }
}
