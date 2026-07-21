contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.30/test/libyul/yulSyntaxTests/dot_consecutive_function_arg.yul
            function x(a..b) {}
        }
    }
}
