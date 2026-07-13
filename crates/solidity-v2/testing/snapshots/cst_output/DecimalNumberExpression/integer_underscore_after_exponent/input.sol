contract C {
    function f() {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/parsing/lexer_numbers_with_underscores_decimal_fail.sol
        12e_34;
    }
}
