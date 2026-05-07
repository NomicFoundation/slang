library Lib {
    using Math for *;
    function test(uint x) internal {
        x.add(1);
    }
}
library Math {
    function add(uint x, uint y) internal {}
}
