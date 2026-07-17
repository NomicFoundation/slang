contract Test {
    enum Severity { Info, Error }

    event Log (
        Severity level,
        string message
    );

    function test() public {
        emit Log({message: "testing", level: Severity.Info});
    }
}
