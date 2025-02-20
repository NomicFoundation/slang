contract Test {
    function test() public {
        ripemd160(); // should bind correctly (Solidity context)
        assembly {
            ripemd160() // should not bind (Yul context)
        }
    }
}
