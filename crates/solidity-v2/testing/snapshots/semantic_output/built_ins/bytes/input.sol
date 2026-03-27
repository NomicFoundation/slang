contract Test {
    function testBytes() public {
        bytes memory b1;
        bytes memory b2;
        bytes memory b3 = bytes.concat(b1, b2);
        b1.length;
    }

    function testString() public {
        string memory s1;
        string memory s2;
        string memory s3 = string.concat(s1, s2);
    }
}
