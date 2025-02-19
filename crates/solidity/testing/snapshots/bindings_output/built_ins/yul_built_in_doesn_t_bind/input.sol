contract Test {
    function test() public {
        mstore(); // should not bind (Solidity context)
        assembly {
            mstore() // should bind correctly (Yul context)
        }
    }
}
