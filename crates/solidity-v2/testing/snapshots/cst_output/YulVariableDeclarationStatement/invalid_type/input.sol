contract C {
    function f() {
        assembly {
            // https://github.com/argotorg/solidity/blob/v0.8.27/test/libyul/yulSyntaxTests/invalid_type2.yul
            let x := 1:invalidType
        }
    }
}
