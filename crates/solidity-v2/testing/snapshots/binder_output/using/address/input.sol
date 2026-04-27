library Address {
    function sendValue(address payable recipient) internal {}
}
contract Test {
    using Address for address payable;
    function test(address _rcpt) public {
        payable(_rcpt).sendValue();
    }
}
