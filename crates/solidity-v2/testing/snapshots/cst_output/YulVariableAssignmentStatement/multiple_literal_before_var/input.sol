contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.29/test/libyul/yulSyntaxTests/multiple_assignment_1.yul
            123, x := f()
        }
    }
}
