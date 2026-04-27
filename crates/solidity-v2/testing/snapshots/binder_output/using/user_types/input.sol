type ShortString is bytes32;

contract Test {
    using ShortStringLib for ShortString;
    using BytesLib for bytes32;

    function test(bytes32 data) public {
        ShortString s;

        ShortString.wrap(data).nop();
        ShortString.unwrap(s).pon();
    }
}

library ShortStringLib {
    function nop(ShortString x) internal {}
}

library BytesLib {
    function pon(bytes32 x) internal {}
}
