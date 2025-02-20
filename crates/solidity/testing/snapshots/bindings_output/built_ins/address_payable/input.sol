contract Test {
    function testPayable(address payable rcpt) public {
        rcpt.transfer(1);
    }
    function testNonPayable(address rcpt) public {
        rcpt.transfer(1);
    }
}
