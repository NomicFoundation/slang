contract BuiltInsTest {
    function testRequire() public {
        require(true, "should always succeed");
    }

    function testRevert() public {
        revert("testing revert");
    }

    function testAssert() public {
        assert(2 + 2 == 4);
    }

    function testMath() public {
        bytes memory x1;

        uint v1 = addmod(1, 2, 3);
        bytes32 v2 = blockhash(1);
        bytes32 v3 = blobhash(2);
        address v4 = ecrecover(v2, 1, v2, v2);
        uint256 v5 = gasleft();
        bytes32 v6 = keccak256(x1);
        uint v7 = mulmod(1, 2, 3);
        bytes20 v8 = ripemd160(x1);
        bytes32 v9 = sha256(x1);
    }

    function testSelfDestruct() public {
        selfdestruct(address(0x0));
    }

    function testAbiFunctions() public {
        bytes memory x1;
        uint v1 = abi.decode(x1, (uint));
        bytes memory v2 = abi.encode(v1);
        bytes memory v3 = abi.encodeCall(this.testMath, (1, 2, 3));
        bytes memory v4 = abi.encodePacked(10, 20);
        bytes memory v5 = abi.encodeWithSelector(this.testMath.selector, (1, 2, 3));
        string memory x2;
        bytes memory v6 = abi.encodeWithSignature(x2, (1, 2, 3));
    }
}
