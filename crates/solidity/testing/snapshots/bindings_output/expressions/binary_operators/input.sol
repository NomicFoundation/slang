library Lib {
    function nop_uint(uint x) public returns (uint) {}
    function nop_bool(bool x) public returns (bool) {}
}

contract Test {
    using Lib for uint;
    using Lib for bool;
    function test(uint a, uint b) public {
        (a += b).nop_uint();
        (true ? a : b).nop_uint();
        (a == b).nop_bool();
        (a > b).nop_bool();
        (a | b).nop_uint();
        (a << 1).nop_uint();
        (a + b).nop_uint();
        (a * b).nop_uint();
        (a++).nop_uint();
        (++a).nop_uint();
    }
}
