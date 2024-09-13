contract B1 {
    //   ^def:1
    function b1_func() returns (int) {
        //   ^def:2
        return 1;
    }
}

contract B2 {
    //   ^def:3
    enum E { E1, E2 }
    //   ^def:7
    function b2_func() returns (int) {
        //   ^def:4
        return 2;
    }
}

contract C is B1 {
    //   ^def:5
    //        ^ref:1
    E e_at_c;
    //<ref:!   --- E should not be directly accessible here
    function c_func() returns (int) {
        //   ^def:6
        return 3 + b1_func() + b2_func();
        //                     ^ref:!   --- B2 is not in our ancestry
        //         ^ref:2
    }
}

contract D is C, B2 {
    //           ^ref:3
    //        ^ref:5
    E e_at_d;
    //<ref:7
    function d_func() returns (int) {
        return c_func() + b1_func() + b2_func();
        //                            ^ref:4
        //                ^ref:2
        //     ^ref:6
    }
}
