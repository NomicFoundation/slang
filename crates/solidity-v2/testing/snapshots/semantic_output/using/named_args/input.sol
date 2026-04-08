contract Test {
    using Utils for uint;

    function test(uint x) public {
        x.operate({a: 1});
        Utils.operate({x: x, a: 1});
    }
}

library Utils {
    function operate(uint x, uint a) public returns (uint) {
        return x;
    }
}
