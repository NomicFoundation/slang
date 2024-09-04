contract Test {
    enum  Severity {
        //^def:1
        Info,
        //<def:3
        Error
    }

    event Log (
        //^def:2
        Severity level,
        //<ref:1
        string message
    );

    function test() {
        emit Log(Severity.Info, "testing");
        //                ^ref:3 (>= 0.4.21)
        //       ^ref:1 (>= 0.4.21)
        //   ^ref:2 (>= 0.4.21)
    }
}
