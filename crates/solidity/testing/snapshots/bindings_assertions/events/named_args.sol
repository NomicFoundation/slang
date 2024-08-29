contract Test {
    enum Severity { Info, Error }
    //   ^def:3

    event Log (
        Severity level,
        //       ^def:1
        string message
        //     ^def:2
    );

    function test() {
        emit Log({message: "testing", level: Severity.Info});
        //                                   ^ref:3 (>= 0.4.21)
        //                            ^ref:1 (>= 0.4.21)
        //        ^ref:2 (>= 0.4.21)
    }
}
