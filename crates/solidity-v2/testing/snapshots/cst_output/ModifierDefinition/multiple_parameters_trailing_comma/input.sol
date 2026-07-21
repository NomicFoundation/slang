contract C {
    // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/multiple_modifier_arg_trailing_comma.sol
    modifier m(uint a, uint b,) { _; }
}
