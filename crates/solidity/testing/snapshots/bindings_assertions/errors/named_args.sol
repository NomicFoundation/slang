contract Test {
    enum Cause { InsuficientFunds, NotAuthorized, InvalidDate }
    //   ^def:3
    error Failure (
        Cause cause,
        //    ^def:1
        string details
        //     ^def:2
    );
    function test() public {
        revert Failure({cause: Cause.NotAuthorized, details: "not owner"});
        //              ^ref:1 (>= 0.8.4)
        //                     ^ref:3 (>= 0.8.4)
        //                                          ^ref:2 (>= 0.8.4)
    }
}
