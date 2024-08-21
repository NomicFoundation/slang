contract Arrays {
    uint[10] ary;
    //       ^def:1

    function test() public {
        uint i = 4;
        //   ^def:2
        uint[3] indices = [1, 2, 3];
        //      ^def:3
        ary[1] = ary[i + indices[1]];
        //               ^ref:3
        //       ^ref:1
        //<ref:1
    }
}
