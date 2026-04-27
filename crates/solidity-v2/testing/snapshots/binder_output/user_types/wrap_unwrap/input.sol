type ShortString is bytes32;

contract Test {
    function test(bytes32 data) public {
        ShortString s = ShortString.wrap(data);
        ShortString.unwrap(s);
    }
}
