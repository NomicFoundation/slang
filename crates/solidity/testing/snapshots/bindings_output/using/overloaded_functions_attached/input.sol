library Lib {
    function add(uint256 x, uint256 y) internal returns (uint256) { return x + y; }
    function add(int256 x, int256 y) internal returns (int256) { return x + y; }
}

contract Test {
    using Lib for uint256;
    using Lib for int256;
    using Lib for uint160;

    function testUsing(uint256 ux, uint256 uy, int256 ix, int256 iy) internal {
        ux.add(uy).add(ux);
        ix.add(iy).add(ix);
    }

    function testConvertible(uint160 ux, uint160 uy) internal {
        Lib.add(ux, uy);
        ux.add(uy).add(ux);
    }
}
