contract C {
    // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/single_return_param_trailing_comma.sol
    function f() internal returns (uint a,) {}
}
