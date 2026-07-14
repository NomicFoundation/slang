contract Foo {
    uint[] m_x;
    function f() public view {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/location_specifiers_for_locals_multi.sol
        uint[] storage memory x = m_x;
    }
}
