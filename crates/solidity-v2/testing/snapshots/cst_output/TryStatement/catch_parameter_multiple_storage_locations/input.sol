contract C {
    function f() public {
        // https://github.com/argotorg/solidity/blob/v0.8.18/test/libsolidity/syntaxTests/tryCatch/multiple_returns_vars_and_catch_parameter_location.sol
        try this.f() {} catch (string memory calldata x) {}
    }
}
