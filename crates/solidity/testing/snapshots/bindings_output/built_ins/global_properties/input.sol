contract Test {
    function testTx() public {
        address origin = tx.origin;
        uint price = tx.gasprice;
    }

    function testBlock() public {
        uint v1 = block.basefee;
        uint v2 = block.blobbasefee;
        uint v3 = block.chainid;
        address v4 = block.coinbase;
        uint v5 = block.difficulty;
        uint v6 = block.gaslimit;
        uint v7 = block.number;
        uint v8 = block.prevrandao;
        uint v9 = block.timestamp;
    }

    function testMsg() public {
        bytes v1 = msg.data;
        address v2 = msg.sender;
        bytes4 v3 = msg.sig;
        uint v4 = msg.value;
    }
}
