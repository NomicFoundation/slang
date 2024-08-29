contract Test {
    enum Severity {
        INFO,
        ERROR
    }

    event Log(
        string name,
        Severity level
    );

    error Failure(
        Severity severity,
        string cause
    );

    function test() public {
        emit Log({level: Severity.INFO, name: "Testing"});
        revert Failure({severity: Severity.ERROR, cause: "Testing"});
    }
}
