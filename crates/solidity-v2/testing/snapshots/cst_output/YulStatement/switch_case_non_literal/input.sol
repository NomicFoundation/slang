contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.0/test/libyul/yulSyntaxTests/switch_invalid_case.yul
            switch 42
            case mul(1, 2) {}
            default {}
        }
    }
}
