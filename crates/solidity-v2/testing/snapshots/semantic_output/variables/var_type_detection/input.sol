contract Test {
    function test() public {
        var x = tx.origin;
        x.transfer(1);

        var (a, b) = (msg.sender, 1);
        a.send(b);
    }
}
