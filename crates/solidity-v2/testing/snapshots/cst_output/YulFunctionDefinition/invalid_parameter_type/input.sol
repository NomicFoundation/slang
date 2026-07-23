contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.27/test/libyul/yulSyntaxTests/invalid_type3.yul
            function f(a: invalidType) -> b {}
        }
    }
}
