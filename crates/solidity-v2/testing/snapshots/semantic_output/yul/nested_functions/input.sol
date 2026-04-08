contract Test {
    uint constant FOO = 1;

    function test() public {
        assembly {
            function outer() {
                function inner1() -> a {
                    a := add(FOO, 0x40)
                }
                let x := inner1()
            }
            outer()
        }
    }
}
