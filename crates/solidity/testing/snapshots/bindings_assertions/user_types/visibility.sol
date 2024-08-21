contract Test {
    //   ^def:dummy
    type Internal is uint;
    //   ^def:2 (>= 0.8.8)

    function test() public returns (int32) {
        Internal inter = Internal.wrap(10);
        //               ^ref:2 (>= 0.8.8)
        //<ref:2 (>= 0.8.8)
        TopLevel tl = TopLevel.wrap(20);
        //            ^ref:1 (>= 0.8.8)
        //<ref:1 (>= 0.8.8)
        return MyLib.LibType.unwrap(MyLib.create());
        //                          ^ref:3
        //           ^ref:4 (>= 0.8.8)
        //     ^ref:3 (>= 0.8.8)
    }
}

type TopLevel is uint256;
//   ^def:1 (>= 0.8.8)

library MyLib {
    //  ^def:3
    type LibType is int32;
    //   ^def:4 (>= 0.8.8)

    function create() public returns (LibType) {
        //                            ^ref:4 (>= 0.8.8)
        return LibType.wrap(30);
        //     ^ref:4 (>= 0.8.8)
    }
}
