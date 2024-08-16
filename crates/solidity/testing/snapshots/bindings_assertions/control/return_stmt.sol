contract Test {
    function test() returns (int) {
        int x = 1;
        //  ^def:1
        return x;
        //     ^ref:1
    }
}
