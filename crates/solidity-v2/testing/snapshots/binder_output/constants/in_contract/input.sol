contract Constants {
    enum Direction { NORTH, SOUTH, EAST, WEST }

    address public constant MY_ADDRESS = 0x777788889999AaAAbBbbCcccddDdeeeEfFFfCcCc;
    uint256 public constant MY_UINT = 123;
    Direction public constant MY_DIRECTION = Direction.EAST;

    function test() public {
        assert(MY_DIRECTION != Direction.NORTH);
        assert(MY_UINT > 100);
        require(msg.sender == MY_ADDRESS);
    }
}
