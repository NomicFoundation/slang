contract Test {
    enum Severity {
        INFO,
        ERROR
    }

    error Failure(
        Severity severity,
        string cause
    );

    function test() public {
        revert Failure(Severity.ERROR, "Testing");
    }
}
