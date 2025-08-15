library Lib {
    function nop(uint256) internal {}
}

contract Test {
    using Lib for uint256;

    struct Point {
        uint256 x;
        uint256 y;
    }

    function test(bytes memory input) internal {
        abi.decode(input, (uint256)).nop();
        abi.decode(input, (uint256[]))[0].nop();
        abi.decode(input, (Point)).x.nop();
        abi.decode(input, (Point[]))[0].x.nop();
        abi.decode(input, (uint256, uint256));
    }
}
