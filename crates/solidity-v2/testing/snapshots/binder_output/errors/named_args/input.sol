contract Test {
    enum Cause { InsuficientFunds, NotAuthorized, InvalidDate }
    error Failure (
        Cause cause,
        string details
    );
    function test() public {
        revert Failure({cause: Cause.NotAuthorized, details: "not owner"});
    }
}
