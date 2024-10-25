contract Test {
    function testAddress(address recipient) public {
        bytes memory x1;
        (bool v1, bytes memory v2) = recipient.call(x1);
        (bool v5, bytes memory v6) = recipient.delegatecall(x1);
        (bool v7, bytes memory v8) = recipient.staticcall(x1);
        recipient.transfer(1);
        bool v9 = recipient.send(1);
        uint256 v10 = address(this).balance;
    }
}
