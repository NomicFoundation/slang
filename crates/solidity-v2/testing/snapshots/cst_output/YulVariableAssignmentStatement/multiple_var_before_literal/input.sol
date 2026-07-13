contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.29/test/libyul/yulSyntaxTests/multiple_assignment_2.yul
            x, 123 := f()
        }
    }
}
