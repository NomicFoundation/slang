contract Test {
    event TestEvent(int id);
    //    ^def:1

    function test_emit() public {
        int x = 1;
        //  ^def:2

        emit TestEvent(x);
        //             ^ref:2 (>= 0.4.21)
        //   ^ref:1 (>= 0.4.21)
        emit Utils.Debug(x);
        //               ^ref:2 (>= 0.4.21)
        //         ^ref:4 (>= 0.4.21)
        //   ^ref:3 (>= 0.4.21)
    }
}

library Utils {
    //  ^def:3
    event Debug(int subject);
    //    ^def:4
}
