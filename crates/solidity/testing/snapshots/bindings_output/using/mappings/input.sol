contract Test {
    using Lib for mapping(address => uint);
    mapping(address => uint) balances;
    function test() public {
        balances.nop();
    }
}
library Lib {
    function nop(mapping(address => uint) storage m) internal {}
}
