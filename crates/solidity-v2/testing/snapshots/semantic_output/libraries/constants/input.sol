library Lib {
    uint private constant X = 1;
    uint public constant Y = 2;

    function test() public returns (uint) {
        return X;
    }
}

contract Test {
    function test() public {
        Lib.Y;
    }
}
