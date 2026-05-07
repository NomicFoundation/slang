contract Test {
    function sub(uint a, uint b) public returns (uint) {
        uint c = a;
        unchecked { return c - b; }
    }
}
