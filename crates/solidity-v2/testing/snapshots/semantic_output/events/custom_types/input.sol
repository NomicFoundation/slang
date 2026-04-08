contract Test {
    enum Severity {
        INFO,
        ERROR
    }

    event Log(
        Severity level,
        string name
    );

    function test() public {
        emit Log(Severity.INFO, "Testing");
    }
}
