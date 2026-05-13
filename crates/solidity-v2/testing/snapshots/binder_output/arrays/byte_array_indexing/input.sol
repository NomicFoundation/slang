library Utils {
    function nop(bytes1) internal {}
}

contract Test {
    using Utils for bytes1;

    function test() public {
        bytes32 seed;
        seed[0].nop();
    }
}
