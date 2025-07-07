contract Test {
    function test(address rcpt, bytes memory data) public {
        // this is valid on Solidity < 0.7.0
        rcpt.call.value(1)(data);
        rcpt.call.gas(1)(data);
        rcpt.call.value(1).gas(1)(data);
    }
}
