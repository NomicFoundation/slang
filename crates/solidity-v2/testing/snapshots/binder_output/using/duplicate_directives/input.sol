library Lib {
    function nop(uint x) internal {}
}

contract Base {
    using Lib for uint;
}

contract Test is Base {
    using Lib for uint;
    using Lib for uint256;

    function test(uint x) internal {
        x.nop();
    }
}
