contract Test {
    enum  Cause {
        //^def:1
        InsuficientFunds,
        NotAuthorized,
        //<def:2
        InvalidDate
    }
    error Failure (
        //^def:3
        Cause cause,
        //<ref:1
        string details
    );
    function test() public {
        revert Failure(Cause.NotAuthorized, "not owner");
        //                   ^ref:2 (>= 0.8.4)
        //             ^ref:1 (>= 0.8.4)
        //     ^ref:3 (>= 0.8.4)
    }
}
