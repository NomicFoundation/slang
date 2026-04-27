contract Test {
    function test(uint amount) public {
        // transfer/send should not bind on >= 0.8.0
        msg.sender.transfer(amount);
        require(msg.sender.send(amount));
    }
}
