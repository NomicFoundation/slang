contract AssemblyBuiltIns {
    function test() public {
        assembly {
            mstore(0x80, byte(10, 31))
            return (0x80,1)
        }
    }
}
