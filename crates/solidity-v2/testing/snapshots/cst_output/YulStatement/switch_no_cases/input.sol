contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.0/test/libyul/yulSyntaxTests/switch_statement_no_access.yul

            switch 42
        }
    }
}
