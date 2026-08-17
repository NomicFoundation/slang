contract Other {
    event Warn(string reason);
}

contract Test {
    event Log(
        string name,
        uint level
    );

    function test() public {
        // Invalid Solidity: an event invocation has to be prefixed by `emit`.
        // The argument names should still resolve to the event parameters.
        Log({level: 1, name: "Testing"});
        Other.Warn({reason: "Testing"});
    }
}
