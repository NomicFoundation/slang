contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.0/test/libyul/yulSyntaxTests/number_literals_3.yul
            let x := 1e5
        }
    }
}
