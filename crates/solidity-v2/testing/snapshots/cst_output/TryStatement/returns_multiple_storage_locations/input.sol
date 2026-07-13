contract C {
    function f() public returns (string memory) {
        // https://github.com/argotorg/solidity/blob/v0.8.18/test/libsolidity/syntaxTests/tryCatch/multiple_returns_vars_and_catch_parameter_location.sol
        try this.f() returns (string memory storage a) {} catch {}
    }
}
