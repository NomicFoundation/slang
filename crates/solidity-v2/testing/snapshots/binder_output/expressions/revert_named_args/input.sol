contract Test {
    error Failure(
        uint severity,
        string cause
    );

    function test() public {
        revert Failure({severity: 100, cause: "Testing"});
    }
}
