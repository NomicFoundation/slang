type ShortString is bytes32;

contract Test {
    using Lib for ShortString;
    using Lib for bytes32;

    function test(bytes32 data) public {
        ShortString s;

        ShortString.wrap(data).nop();
        ShortString.unwrap(s).pon();
    }
}

library Lib {
    function nop(ShortString x) internal {}
    function pon(bytes32 x) internal {}
}
