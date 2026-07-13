contract Foo {
    // https://github.com/argotorg/solidity/blob/v0.8.18/test/libsolidity/syntaxTests/parsing/location_specifiers_for_fn_returns_multi.sol
    function f() returns (string calldata memory) {}
}
