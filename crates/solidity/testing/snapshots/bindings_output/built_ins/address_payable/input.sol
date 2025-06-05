contract Test {
    function testPayable(address payable rcpt) public {
        rcpt.transfer(1);
    }
    function testNonPayable(address rcpt) public {
        // non payable address shoudln't bind transfer after 0.5.0, but...
        rcpt.transfer(1);
        // this type casting syntax is allowed by solc
        address(uint160(rcpt)).transfer(1);
    }
}
