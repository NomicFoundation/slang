contract Test {
    uint[] a;
    function testArray() public {
        uint[] storage b = new uint[](5);
        assert(b.length == 5);

        a.push();
        a.push(1);
        a.pop();
    }

    function testConcat() public {
        bytes memory b1;
        bytes memory b2;
        bytes memory b3 = b1.concat(b2);

        string memory s1;
        string memory s2;
        string memory s3 = s1.concat(s2);
    }
}
