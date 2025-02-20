contract Test {
    uint d;
    function test(uint a) public returns (uint b) {
        uint c;
        assembly {
            c := a
            b := c
            c := d.slot
        }
    }
}
