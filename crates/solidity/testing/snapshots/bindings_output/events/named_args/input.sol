contract Test {
    enum Severity { Info, Error }

    event Log (
        Severity level,
        string message
    );

    function test() {
        emit Log({message: "testing", level: Severity.Info});
    }
}
