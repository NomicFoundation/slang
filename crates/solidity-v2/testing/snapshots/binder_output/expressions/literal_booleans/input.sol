library Lib {
    function nop(bool x) internal {}
}
contract Test {
    using Lib for bool;
    function test() public {
        true.nop();
        false.nop();
    }
}
