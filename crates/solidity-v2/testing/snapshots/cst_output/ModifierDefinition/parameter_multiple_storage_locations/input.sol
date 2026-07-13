contract A {
    // https://github.com/argotorg/solidity/blob/v0.8.18/test/libsolidity/syntaxTests/modifiers/multiple_parameter_location.sol
    modifier mod(string storage memory a) { _; }
}
