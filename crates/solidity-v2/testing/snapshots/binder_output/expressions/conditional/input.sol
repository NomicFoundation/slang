library Lib {
    function nop(uint) internal {}
}

contract Test {
    using Lib for uint;
    function test(uint x) internal {
        (true ? 0 : x).nop();
    }
}
