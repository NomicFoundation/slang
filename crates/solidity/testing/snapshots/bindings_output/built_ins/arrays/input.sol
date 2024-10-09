contract Test {
    uint[] a;
    function testArray() public {
        uint[] memory b = new uint[](5);
        assert(b.length == 5);

        a.push();
        a.push(1);
        a.pop();
    }
}
