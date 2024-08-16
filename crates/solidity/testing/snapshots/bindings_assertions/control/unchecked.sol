contract Test {
    function sub(uint a, uint b) public returns (uint) {
        //                    ^def:2
        //            ^def:1
        uint c = a;
        //       ^ref:1
        //   ^def:3
        unchecked { return c - b; }
        //                     ^ref:2 (>= 0.8.0)
        //                 ^ref:3 (>= 0.8.0)
    }
}
