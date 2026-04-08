library Utils {
    struct Point {
        uint x;
        uint y;
    }

    function add(Point memory, Point memory) internal returns (Point memory) {}
    function add(uint, uint) internal returns (uint) {}
}

contract Test {
    using Utils for uint;
    using Utils for Utils.Point;

    function test(Utils.Point calldata x) public {
        Utils.Point memory p;
        p.add(x).add(x);
    }
}
