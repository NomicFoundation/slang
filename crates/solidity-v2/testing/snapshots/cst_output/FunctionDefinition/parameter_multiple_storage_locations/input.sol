contract Foo {
    // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/location_specifiers_for_params_multi.sol
    function f(uint[] storage memory x) internal {}
}
