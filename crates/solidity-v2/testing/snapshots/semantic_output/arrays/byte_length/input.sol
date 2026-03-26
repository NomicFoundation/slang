contract Test {
    function test() public {
        byte data;
        data.length;  // .length should bind in Solidity < 0.8.0
    }
}
