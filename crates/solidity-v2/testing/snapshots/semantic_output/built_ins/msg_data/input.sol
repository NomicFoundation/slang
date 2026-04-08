library Lib {
    function nop(bytes calldata) internal {}
}

contract Test {
    using Lib for bytes;
    function test() internal {
        msg.data.nop();
    }
}
