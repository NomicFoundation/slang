interface External {
    function sample(uint x) external payable returns (uint);
    //       ^def:5
}

contract Test {
    External ext;
    //       ^def:4
    function test(uint x) public returns (uint) {
        //             ^def:1
        uint v = 10;
        //   ^def:2
        uint g = 800;
        //   ^def:3

        return ext.sample{ value: v, gas: g }(x);
        //                                    ^ref:1 (>= 0.6.2)
        //                                ^ref:3 (>= 0.6.2)
        //                        ^ref:2 (>= 0.6.2)
        //         ^ref:5
        //     ^ref:4
    }
}
