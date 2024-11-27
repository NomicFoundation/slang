library Lib {
    function squared(int x) public returns (int) { return x * x; }
    //       ^def:1
}

contract Base {
    using Lib for int;
}

contract Test is Base {
    function test(int x) public returns (int) {
        return x.squared();
        //       ^ref:1 (< 0.7.0)
        //       ^ref:! (>= 0.7.0)
    }
}
